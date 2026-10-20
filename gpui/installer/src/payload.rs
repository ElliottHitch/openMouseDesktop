//! The main app's already-built `.exe`, embedded at build time (see
//! `build.rs`), so installing only ever means writing out one file — the
//! app ships self-contained (JS driver bundle + UI assets compiled in),
//! confirmed against `gpui/src/hid.rs`'s `include_str!` of the JS bundle.

pub const APP_EXE_NAME: &str = "openmouse-gpui.exe";
pub const APP_EXE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/openmouse-gpui.exe"));
