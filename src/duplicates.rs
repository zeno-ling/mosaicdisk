//! Content-based duplicate file detection: size buckets → XXH3 head/tail
//! pre-filter → full BLAKE3 confirmation. Files that only exist in an imported
//! CSV (and are no longer on disk) are skipped silently.

use crate::model::{NodeId, Tree};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Progress for a running duplicate search.
#[derive(Default)]
pub struct DupProgress {
    pub done: AtomicU64,
    pub total: AtomicU64,
    pub cancel: AtomicBool,
}

impl DupProgress {
    pub fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

/// A set of files with identical content. `files` is sorted by NodeId so the
/// first entry is a stable "keeper" candidate.
#[derive(Clone, Debug)]
pub struct DupGroup {
    pub size: u64,
    pub files: Vec<NodeId>,
}

impl DupGroup {
    /// Reclaimable bytes if every copy but one is removed.
    pub fn reclaimable(&self) -> u64 {
        self.size * self.files.len().saturating_sub(1) as u64
    }
}

pub struct DupResult {
    pub groups: Vec<DupGroup>,
    #[allow(dead_code)]
    pub scanned: u64,
    pub elapsed: f64,
    pub cancelled: bool,
}

const QUICK_CHUNK: usize = 64 * 1024;

/// Read the first and last `QUICK_CHUNK` bytes of `path` and hash them together
/// with the file length. Identical files always produce the same value; the
/// converse is not guaranteed, which is why BLAKE3 confirms every candidate.
fn quick_hash(path: &std::path::Path) -> std::io::Result<u64> {
    let mut f = File::open(path)?;
    let len = f.metadata()?.len();
    let mut buf = vec![0u8; QUICK_CHUNK];
    let mut h = twox_hash::XxHash3_64::with_seed(0x9E37_79B9_7F4A_7C15);
    let mut read = f.read(&mut buf)?;
    h.write(&buf[..read]);
    if len > QUICK_CHUNK as u64 * 2 {
        f.seek(SeekFrom::End(-(QUICK_CHUNK as i64)))?;
        read = f.read(&mut buf)?;
        h.write(&buf[..read]);
    } else if len > QUICK_CHUNK as u64 {
        // Middle region not covered by head/tail still matters: read the rest
        // of the file from after the head chunk to avoid false negatives.
        loop {
            read = f.read(&mut buf)?;
            if read == 0 {
                break;
            }
            h.write(&buf[..read]);
        }
    }
    h.write(&len.to_le_bytes());
    Ok(h.finish())
}

fn full_hash(path: &std::path::Path) -> std::io::Result<[u8; 32]> {
    let mut f = File::open(path)?;
    let mut h = blake3::Hasher::new();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(*h.finalize().as_bytes())
}

/// Find duplicate files in `t`. Only non-directory, non-deleted nodes with
/// `size >= min_size` are considered.
pub fn find_duplicates(t: &Tree, min_size: u64, progress: Arc<DupProgress>) -> DupResult {
    let start = std::time::Instant::now();
    let mut by_size: HashMap<u64, Vec<NodeId>> = HashMap::new();
    for &id in &t.files_by_size {
        let n = &t.nodes[id as usize];
        if n.is_deleted() || n.size < min_size || n.size == 0 {
            continue;
        }
        by_size.entry(n.size).or_default().push(id);
    }
    // Only sizes with at least two candidates can contain duplicates.
    by_size.retain(|_, v| v.len() >= 2);
    let candidates: Vec<NodeId> = by_size.values().flat_map(|v| v.iter().copied()).collect();
    progress.total.store(candidates.len() as u64, Ordering::Relaxed);

    // Stage 1: XXH3 over head/tail; group candidates that look alike.
    let quick: Vec<(NodeId, u64)> = candidates
        .par_iter()
        .filter_map(|&id| {
            if progress.cancelled() {
                return None;
            }
            let p = t.path(id);
            let h = quick_hash(std::path::Path::new(&p)).ok()?;
            progress.done.fetch_add(1, Ordering::Relaxed);
            Some((id, h))
        })
        .collect();
    let mut by_quick: HashMap<(u64, u64), Vec<NodeId>> = HashMap::new();
    for (id, h) in quick {
        by_quick.entry((t.nodes[id as usize].size, h)).or_default().push(id);
    }
    by_quick.retain(|_, v| v.len() >= 2);

    // Stage 2: full BLAKE3 verification of the surviving candidates.
    let second: Vec<NodeId> = by_quick.values().flat_map(|v| v.iter().copied()).collect();
    progress.total.store(progress.total.load(Ordering::Relaxed) + second.len() as u64, Ordering::Relaxed);
    let exact: Vec<(NodeId, [u8; 32])> = second
        .par_iter()
        .filter_map(|&id| {
            if progress.cancelled() {
                return None;
            }
            let p = t.path(id);
            let h = full_hash(std::path::Path::new(&p)).ok()?;
            progress.done.fetch_add(1, Ordering::Relaxed);
            Some((id, h))
        })
        .collect();
    let mut by_exact: HashMap<(u64, [u8; 32]), Vec<NodeId>> = HashMap::new();
    for (id, h) in exact {
        by_exact.entry((t.nodes[id as usize].size, h)).or_default().push(id);
    }
    let mut groups: Vec<DupGroup> = by_exact
        .into_iter()
        .filter(|(_, v)| v.len() >= 2)
        .map(|((size, _), mut v)| {
            v.sort_unstable();
            DupGroup { size, files: v }
        })
        .collect();
    groups.sort_by(|a, b| b.reclaimable().cmp(&a.reclaimable()).then_with(|| b.size.cmp(&a.size)));
    let scanned = progress.done.load(Ordering::Relaxed);
    DupResult {
        groups,
        scanned,
        elapsed: start.elapsed().as_secs_f64(),
        cancelled: progress.cancelled(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_duplicates_groups_identical_files() {
        let dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("target")
            .join(format!("md-dup-integration-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let data: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        let other: Vec<u8> = (0..200_000u32).map(|i| (i % 253) as u8).collect();
        std::fs::write(dir.join("a.bin"), &data).unwrap();
        std::fs::write(dir.join("b.bin"), &data).unwrap();
        std::fs::write(dir.join("c.bin"), &other).unwrap();

        let mut t = Tree::new();
        let root = t.add_node(
            crate::model::NONE,
            &dir.to_string_lossy(),
            crate::model::Node { flags: crate::model::flags::DIR | crate::model::flags::ROOT, ..Default::default() },
        );
        t.roots.push(root);
        t.add_node(root, "a.bin", crate::model::Node { size: 200_000, ..Default::default() });
        t.add_node(root, "b.bin", crate::model::Node { size: 200_000, ..Default::default() });
        t.add_node(root, "c.bin", crate::model::Node { size: 200_000, ..Default::default() });
        t.finalize();

        let res = find_duplicates(&t, 1, Arc::new(DupProgress::default()));
        assert!(!res.cancelled);
        assert_eq!(res.groups.len(), 1, "只有 a.bin 与 b.bin 内容相同");
        let g = &res.groups[0];
        assert_eq!(g.files.len(), 2);
        assert_eq!(g.size, 200_000);
        assert_eq!(g.reclaimable(), 200_000);
        let names: Vec<_> = g.files.iter().map(|&id| t.name(id)).collect();
        assert!(names.contains(&"a.bin") && names.contains(&"b.bin"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn quick_hash_is_stable_and_size_sensitive() {
        let dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("target")
            .join(format!("md-dup-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.bin");
        let b = dir.join("b.bin");
        let c = dir.join("c.bin");
        let data: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        std::fs::write(&a, &data).unwrap();
        std::fs::write(&b, &data).unwrap();
        std::fs::write(&c, &data[..data.len() - 1]).unwrap();
        let ha = quick_hash(&a).unwrap();
        assert_eq!(ha, quick_hash(&b).unwrap());
        assert_ne!(ha, quick_hash(&c).unwrap());
        assert_eq!(full_hash(&a).unwrap(), full_hash(&b).unwrap());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
