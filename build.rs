extern crate winres;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        if cfg!(unix) {
            res.set_toolkit_path("/usr/x86_64-w64-mingw32/bin");
            res.set_ar_path("ar");
            res.set_windres_path("/usr/bin/x86_64-w64-mingw32-windres");
        }

        res.set_icon("icon.ico")
            .set_language(0x0409)
            .set_manifest_file("manifest.xml");
        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
