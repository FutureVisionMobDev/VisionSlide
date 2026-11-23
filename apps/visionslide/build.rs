//! Windows only: embed the app icon and version info (VERSIONINFO) into `visionslide.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `VISIONSLIDE_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/visionslide.ico");
    println!("cargo:rerun-if-env-changed=VISIONSLIDE_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/visionslide.ico")
        .set("ProductName", "VisionSlide")
        .set("FileDescription", "VisionSlide presentations")
        .set("CompanyName", "VisionOffice")
        .set("LegalCopyright", "Copyright (c) the VisionSlide authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "visionslide.exe")
        .set("InternalName", "visionslide");
    if let Err(e) = res.compile() {
        if std::env::var_os("VISIONSLIDE_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=visionslide.exe built without icon/version resources: {e}");
    }
}
