//! Add/Remove Programs entry (`HKCU\...\Uninstall\OpenMouseDesktop`) — same
//! `winreg` crate and per-user `HKEY_CURRENT_USER` pattern already used by
//! `gpui/src/autostart.rs` for the Run-key entry.

#[cfg(windows)]
mod imp {
    use crate::{paths, payload};
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    const UNINSTALL_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\OpenMouseDesktop";

    pub fn write() -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _disposition) = hkcu.create_subkey(UNINSTALL_KEY).map_err(|e| e.to_string())?;

        let install_dir = paths::install_dir().to_string_lossy().into_owned();
        let app_exe = paths::app_exe_path().to_string_lossy().into_owned();
        let uninstaller = paths::uninstaller_exe_path().to_string_lossy().into_owned();
        // KB, rounded up to at least 1 so an empty/odd build never reports 0.
        let size_kb = ((payload::APP_EXE.len() as u32) / 1024).max(1);

        key.set_value("DisplayName", &paths::APP_DISPLAY_NAME.to_string()).map_err(|e| e.to_string())?;
        key.set_value("DisplayVersion", &paths::APP_VERSION.to_string()).map_err(|e| e.to_string())?;
        key.set_value("Publisher", &"OpenMouse Project".to_string()).map_err(|e| e.to_string())?;
        key.set_value("InstallLocation", &install_dir).map_err(|e| e.to_string())?;
        key.set_value("DisplayIcon", &format!("{app_exe},0")).map_err(|e| e.to_string())?;
        key.set_value("UninstallString", &format!("\"{uninstaller}\" {}", crate::UNINSTALL_ARG)).map_err(|e| e.to_string())?;
        key.set_value("NoModify", &1u32).map_err(|e| e.to_string())?;
        key.set_value("NoRepair", &1u32).map_err(|e| e.to_string())?;
        key.set_value("EstimatedSize", &size_kb).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn remove() -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        match hkcu.delete_subkey_all(UNINSTALL_KEY) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn write() -> Result<(), String> {
        Ok(())
    }
    pub fn remove() -> Result<(), String> {
        Ok(())
    }
}

pub use imp::{remove, write};
