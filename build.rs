/// 把 assets/app_icon.ico 嵌入 Windows PE 资源，使资源管理器、任务栏、
/// 快捷方式显示应用图标。eframe 的 `.with_icon()` 只设置运行时窗口图标，
/// 覆盖不到 explorer 读取的 PE RT_ICON/RT_GROUP_ICON 资源。
fn main() {
    println!("cargo:rerun-if-changed=assets/app_icon.ico");
    println!("cargo:rerun-if-env-changed=WINDRES");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out = std::env::var("OUT_DIR").expect("OUT_DIR");
    let rc = std::path::Path::new(&out).join("app_icon.rc");
    let ico = std::path::Path::new("assets/app_icon.ico")
        .canonicalize()
        .expect("assets/app_icon.ico not found");
    // .rc 里路径用正斜杠，避免反斜杠被当作转义。
    let ico_fwd = ico.to_string_lossy().replace('\\', "/");
    std::fs::write(
        &rc,
        format!("1 ICON \"{}\"\n", ico_fwd),
    )
    .expect("write rc");

    let compiler = std::env::var("WINDRES").unwrap_or_else(|_| "windres".into());
    let res = std::path::Path::new(&out).join("app_icon.res");
    // windres is optional: CI (MSVC toolchain) and contributors without MinGW
    // can still build. The icon is embedded only when a working windres is
    // available; release packaging always provides one via PATH/WINDRES.
    let status = std::process::Command::new(&compiler)
        .arg("-i")
        .arg(&rc)
        .arg("-o")
        .arg(&res)
        .arg("-O")
        .arg("coff")
        .status();
    match status {
        Ok(s) if s.success() => println!("cargo:rustc-link-arg={}", res.display()),
        Ok(s) => println!("cargo:warning=windres exited with {s}; skipping PE icon embedding"),
        Err(e) => println!("cargo:warning=windres unavailable ({e}); skipping PE icon embedding (set WINDRES to its path to embed)"),
    }
}
