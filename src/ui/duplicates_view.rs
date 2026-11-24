//! Duplicates tab: grouped, checkable results of a content-based search.

use super::{App, Cmd, chrome, icons};
use crate::duplicates::DupGroup;
use crate::format;
use crate::i18n::tr;
use crate::model::NodeId;
use crate::theme;
use eframe::egui::{self, RichText, Ui};
use std::collections::HashSet;
use std::sync::atomic::Ordering;

#[derive(Default)]
pub struct DupViewState {
    pub groups: Vec<DupGroup>,
    pub checked: HashSet<NodeId>,
    pub expanded: HashSet<usize>,
    pub searched: bool,
}

impl DupViewState {
    /// Reset checked state: keep one copy (the first) per group, select the rest.
    pub fn set_groups(&mut self, mut groups: Vec<DupGroup>) {
        self.checked.clear();
        self.expanded.clear();
        for g in &mut groups {
            for &id in g.files.iter().skip(1) {
                self.checked.insert(id);
            }
        }
        self.groups = groups;
        self.searched = true;
    }

    pub fn clear(&mut self) {
        self.groups.clear();
        self.checked.clear();
        self.expanded.clear();
    }
}

pub fn show(app: &mut App, ui: &mut Ui) {
    let ctx = ui.ctx().clone();
    let pal = app.pal;
    let fmt = app.fmt();
    let full = ui.max_rect();
    ui.painter().rect(full, egui::CornerRadius::same(12), pal.card, egui::Stroke::new(1.0, pal.separator), egui::StrokeKind::Inside);

    // Toolbar
    let bar = egui::Rect::from_min_max(full.min + egui::vec2(14.0, 8.0), egui::pos2(full.max.x - 14.0, full.min.y + 46.0));
    {
        let mut c = ui.new_child(egui::UiBuilder::new().max_rect(bar).layout(egui::Layout::left_to_right(egui::Align::Center)));
        c.spacing_mut().item_spacing.x = 12.0;
        c.label(RichText::new(tr("Minimum file size")).color(pal.text2));
        egui::ComboBox::from_id_salt("dupminsize")
            .selected_text(if app.s.dup_min_size == 0 { tr("All").to_string() } else { fmt.size(app.s.dup_min_size) })
            .width(90.0)
            .show_ui(&mut c, |ui| {
                for v in [0u64, 1024 * 1024, 10 * 1024 * 1024, 100 * 1024 * 1024, 1024 * 1024 * 1024] {
                    let l = if v == 0 { tr("All").to_string() } else { fmt.size(v) };
                    ui.selectable_value(&mut app.s.dup_min_size, v, l);
                }
            });
        if app.dup_job.is_some() {
            if chrome::primary_button(&mut c, &pal, "Cancel", icons::stop).clicked() {
                app.run(Cmd::CancelDuplicates, &ctx);
            }
        } else if chrome::primary_button(&mut c, &pal, "Find Duplicates", icons::search).clicked() {
            app.run(Cmd::FindDuplicates, &ctx);
        }
        if !app.dup_view.checked.is_empty() && chrome::primary_button(&mut c, &pal, "Move to Trash", icons::warning).clicked() {
            app.run(Cmd::DeleteDuplicates, &ctx);
        }
        c.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some(job) = &app.dup_job {
                let done = job.progress.done.load(Ordering::Relaxed);
                let total = job.progress.total.load(Ordering::Relaxed);
                let txt = if total > 0 {
                    format!("{} / {}", format::thousands(done), format::thousands(total))
                } else {
                    tr("Preparing…").to_string()
                };
                ui.label(RichText::new(txt).color(pal.text3).font(theme::regular(11.5)));
            } else if app.dup_view.searched {
                let n = app.dup_view.groups.len();
                let reclaim: u64 = app.dup_view.groups.iter().map(|g| g.reclaimable()).sum();
                let txt = format!("{} {} · {} {}", format::thousands(n as u64), tr("groups"), fmt.size(reclaim), tr("reclaimable"));
                ui.label(RichText::new(txt).color(pal.text3).font(theme::regular(11.5)));
            }
        });
    }

    let list = egui::Rect::from_min_max(egui::pos2(full.min.x + 2.0, bar.max.y + 4.0), full.max - egui::vec2(2.0, 4.0));
    let mut c = ui.new_child(egui::UiBuilder::new().max_rect(list));
    egui::ScrollArea::vertical().auto_shrink(false).show(&mut c, |ui| {
        if app.dup_job.is_none() && app.dup_view.groups.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);
                ui.label(RichText::new(tr("No duplicate groups found yet")).color(pal.text2).font(theme::semibold(16.0)));
                ui.add_space(8.0);
                ui.label(RichText::new(tr("Click Find Duplicates to scan the current results by file content.")).color(pal.text3));
            });
            return;
        }
        let groups = app.dup_view.groups.clone();
        let t = app.tree.as_ref();
        let keep_label = tr("keep");
        for (gi, g) in groups.iter().enumerate() {
            let expanded = app.dup_view.expanded.contains(&gi);
            ui.push_id(gi, |ui| {
                let head = ui.horizontal(|ui| {
                    let mut check = g.files.iter().skip(1).all(|&id| app.dup_view.checked.contains(&id));
                    if ui.checkbox(&mut check, "").clicked() {
                        for &id in g.files.iter().skip(1) {
                            if check { app.dup_view.checked.insert(id); } else { app.dup_view.checked.remove(&id); }
                        }
                    }
                    if ui.button(if expanded { "▾" } else { "▸" }).clicked() {
                        if expanded { app.dup_view.expanded.remove(&gi); } else { app.dup_view.expanded.insert(gi); }
                    }
                    let txt = format!(
                        "{} {} · {} {} · {} {}",
                        format::thousands(g.files.len() as u64),
                        tr("files"),
                        fmt.size(g.size),
                        tr("each"),
                        fmt.size(g.reclaimable()),
                        tr("reclaimable")
                    );
                    ui.label(RichText::new(txt).font(theme::medium(13.0)));
                });
                head.response.on_hover_text(tr("Group of identical files"));
                if expanded {
                    ui.indent("dupgroup", |ui| {
                        if let Some(t) = t {
                            for (k, &id) in g.files.iter().enumerate() {
                                let mut check = app.dup_view.checked.contains(&id);
                                if ui.checkbox(&mut check, "").clicked() {
                                    if check { app.dup_view.checked.insert(id); } else { app.dup_view.checked.remove(&id); }
                                }
                                let keep = if k == 0 { format!(" ({keep_label})") } else { String::new() };
                                let p = t.path(id);
                                ui.label(RichText::new(format!("{p}{keep}")).color(pal.text2).font(theme::regular(12.0)));
                            }
                        }
                    });
                }
            });
            ui.separator();
        }
    });
}
