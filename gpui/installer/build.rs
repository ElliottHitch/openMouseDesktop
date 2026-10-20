use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../assets/icon.ico");
        res.set("ProductName", "OpenMouse Desktop Setup");
        if let Err(error) = res.compile() {
            println!("cargo:warning=could not embed the app icon: {error}");
        }
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));
    // Overridable so a developer can point this at a differently-located
    // build without first `cargo build --release`-ing gpui/ into the
    // default spot (CI always uses the default: it builds gpui/ first).
    let app_exe = env::var("OPENMOUSE_GPUI_EXE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../target/release/openmouse-gpui.exe"));

    fs::copy(&app_exe, out_dir.join("openmouse-gpui.exe")).unwrap_or_else(|error| {
        panic!(
            "could not find the built app at {} ({error}) — run `cargo build --release` in gpui/ \
             first, or set OPENMOUSE_GPUI_EXE to its path",
            app_exe.display()
        )
    });
    println!("cargo:rerun-if-changed={}", app_exe.display());
}
