//! The app's brand green (the `logo.png` pill, `[data-theme="emerald"]` in
//! `src/App.css`) rather than the default Matt dark theme's orange — this is
//! a standalone installer, not a copy of the main app's current theme choice,
//! so it paints the one color scheme that's unambiguously "OpenMouse" no
//! matter what theme the user picked inside the app.
//!
//! Duplicated here rather than depending on the `openmouse-gpui` crate —
//! its `build.rs` needs node/esbuild for the JS driver bundle, which this
//! installer has no use for.

use gpui_kit::component::Theme;
use gpui_kit::gpui::{rgb, App, Hsla};

fn c(hex: u32) -> Hsla {
    rgb(hex).into()
}

pub fn background() -> Hsla {
    c(0x05080a)
}
pub fn surface() -> Hsla {
    c(0x0c1113)
}
pub fn border() -> Hsla {
    c(0x243138)
}
pub fn foreground() -> Hsla {
    c(0xf2f7f4)
}
pub fn muted() -> Hsla {
    c(0x83938a)
}
pub fn accent() -> Hsla {
    c(0x58c97c)
}
pub fn accent_ink() -> Hsla {
    c(0x06140b)
}

/// Paints the live [`Theme`] with the palette above, so every `gpui-kit`
/// component (`Button::primary()`, `Checkbox`, `Spinner`) picks it up
/// automatically instead of needing per-element color overrides — same
/// mechanism `gpui/src/theme.rs`'s `paint_shared` uses for the main app.
/// Call once, after the window exists (`Theme::change` mid-construction
/// poisons first render).
pub fn paint(cx: &mut App) {
    Theme::update(cx, |theme| {
        let colors = &mut theme.colors;
        colors.background = background();
        colors.foreground = foreground();
        colors.muted = surface();
        colors.muted_foreground = muted();
        colors.sidebar = surface();
        colors.border = border();
        colors.accent = accent();
        colors.accent_foreground = accent_ink();
        colors.primary = accent();
        colors.primary_foreground = accent_ink();
        colors.title_bar = surface();
        colors.title_bar_border = border();
    });
}
