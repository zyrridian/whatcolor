fn main() {
    // Only run on Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        return;
    }

    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.set("ProductName", "whatcolor");
    res.set("FileDescription", "Instant screen color picker");
    res.set("LegalCopyright", "MIT License");
    res.compile().expect("Failed to compile Windows resources");
}
