//! Start Menu + optional Desktop `.lnk` creation via `mslnk` — a pure-Rust
//! `.lnk` writer, deliberately chosen over `windows-rs`'s `IShellLink` COM
//! API to match the repo's existing "no windows crate" stance (see
//! `gpui/src/native_window/win.rs`).

#[cfg(windows)]
mod imp {
    use crate::paths;
    use std::path::Path;

    fn write_lnk(target: &Path, link: &Path) -> Result<(), String> {
        if let Some(dir) = link.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let shell_link = mslnk::ShellLink::new(target).map_err(|e| e.to_string())?;
        shell_link.create_lnk(link).map_err(|e| e.to_string())
    }

    pub fn create_start_menu() -> Result<(), String> {
        let link = paths::start_menu_shortcut_path().ok_or("no Start Menu folder")?;
        write_lnk(&paths::app_exe_path(), &link)
    }

    pub fn create_desktop() -> Result<(), String> {
        let link = paths::desktop_shortcut_path().ok_or("no Desktop folder")?;
        write_lnk(&paths::app_exe_path(), &link)
    }

    pub fn remove_start_menu() {
        if let Some(link) = paths::start_menu_shortcut_path() {
            let _ = std::fs::remove_file(link);
        }
    }

    pub fn remove_desktop() {
        if let Some(link) = paths::desktop_shortcut_path() {
            let _ = std::fs::remove_file(link);
        }
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn create_start_menu() -> Result<(), String> {
        Ok(())
    }
    pub fn create_desktop() -> Result<(), String> {
        Ok(())
    }
    pub fn remove_start_menu() {}
    pub fn remove_desktop() {}
}

pub use imp::{create_desktop, create_start_menu, remove_desktop, remove_start_menu};
