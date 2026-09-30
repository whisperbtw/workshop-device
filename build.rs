fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/app.ico")
            .set("ProductName", "Workshop Device")
            .set("FileDescription", "Project Zomboid Workshop downloader")
            .set("OriginalFilename", "Workshop-Device.exe")
            .set("LegalCopyright", "Copyright (c) 2026 whisperbtw")
            .compile()
            .expect("Could not compile the Windows application icon and metadata");
    }
}
