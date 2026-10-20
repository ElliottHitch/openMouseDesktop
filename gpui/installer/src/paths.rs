//! Per-user install location and shortcut paths — no admin rights needed
//! anywhere in this crate.

use std::path::PathBuf;

pub const APP_DISPLAY_NAME: &str = "OpenMouse Desktop";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const UNINSTALL_EXE_NAME: &str = "openmouse-uninstaller.exe";

/// `%LOCALAPPDATA%\Programs\OpenMouse Desktop` on Windows.
pub fn install_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("Programs")
        .join(APP_DISPLAY_NAME)
}

pub fn app_exe_path() -> PathBuf {
    install_dir().join(crate::payload::APP_EXE_NAME)
}

pub fn uninstaller_exe_path() -> PathBuf {
    install_dir().join(UNINSTALL_EXE_NAME)
}

/// `%APPDATA%\Microsoft\Windows\Start Menu\Programs\OpenMouse Desktop.lnk`.
pub fn start_menu_shortcut_path() -> Option<PathBuf> {
    dirs::data_dir().map(|appdata| {
        appdata
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join(format!("{APP_DISPLAY_NAME}.lnk"))
    })
}

pub fn desktop_shortcut_path() -> Option<PathBuf> {
    dirs::desktop_dir().map(|dir| dir.join(format!("{APP_DISPLAY_NAME}.lnk")))
}
