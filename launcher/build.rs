fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();
        // En cross-compilation depuis Linux (WSL), winres utilise "windres" par défaut
        // mais l'outil MinGW-w64 s'appelle x86_64-w64-mingw32-windres.
        if cfg!(target_os = "linux") {
            res.set_windres_path("x86_64-w64-mingw32-windres");
        }
        res.set("ProductName",     env!("CARGO_PKG_NAME"));
        res.set("FileDescription", env!("CARGO_PKG_DESCRIPTION"));
        res.set("FileVersion",     env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion",  env!("CARGO_PKG_VERSION"));
        res.compile().expect("winres: failed to compile resources");
    }
}
