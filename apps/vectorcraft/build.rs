// Modificado por Grupo MOST para Most Vector. Obra original: VectorCraft del equipo ArtCraft (https://github.com/storytold/vectorcraft), Apache-2.0.
//! Windows only: embed the app icon and version info (VERSIONINFO) into `vectorcraft.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `MOST_VECTOR_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/vectorcraft.ico");
    println!("cargo:rerun-if-env-changed=MOST_VECTOR_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/vectorcraft.ico")
        .set("ProductName", "Most Vector")
        .set("FileDescription", "Most Vector vector illustration editor")
        .set("CompanyName", "Grupo MOST")
        .set("LegalCopyright", "Copyright (c) the VectorCraft authors; modificaciones Copyright (c) 2026 Grupo MOST. MIT OR Apache-2.0.")
        .set("OriginalFilename", "vectorcraft.exe")
        .set("InternalName", "vectorcraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("MOST_VECTOR_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=vectorcraft.exe built without icon/version resources: {e}");
    }
    Ok(())
}
