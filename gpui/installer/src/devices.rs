//! Finds the supported devices that are plugged in, so the installer can say
//! what OpenMouse will work with before anything is installed.
//!
//! Detection lives in the app itself, so rather than duplicate it the
//! installer writes the embedded app to a temporary folder and runs it with
//! `--list-supported <file>` (see `gpui/src/supported_devices.rs`), which lists
//! the devices exactly as the Overview page would and exits.

use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::payload;

/// How long to wait for the app before giving up on the list.
const TIMEOUT: Duration = Duration::from_secs(25);
/// Mirrors `gpui/src/supported_devices.rs`'s `ARG`.
const LIST_ARG: &str = "--list-supported";

#[derive(Debug, Clone, Deserialize)]
pub struct Device {
    pub name: String,
    pub brand: String,
}

/// Blocking; run it off the UI thread.
pub fn scan() -> Result<Vec<Device>, String> {
    let dir = std::env::temp_dir().join(format!("openmouse-scan-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let result = run(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    result
}

fn run(dir: &std::path::Path) -> Result<Vec<Device>, String> {
    let exe = dir.join(payload::APP_EXE_NAME);
    let out = dir.join("devices.json");
    std::fs::write(&exe, payload::APP_EXE).map_err(|e| e.to_string())?;

    let mut command = std::process::Command::new(&exe);
    command
        .arg(LIST_ARG)
        .arg(&out)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let mut child = command.spawn().map_err(|e| e.to_string())?;

    let started = Instant::now();
    loop {
        match child.try_wait().map_err(|e| e.to_string())? {
            Some(_) => break,
            None if started.elapsed() > TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("timed out".to_string());
            }
            None => std::thread::sleep(Duration::from_millis(100)),
        }
    }

    let text = std::fs::read_to_string(&out).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}
