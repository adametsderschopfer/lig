//! Apply the native window material through Iced's window callback.

#[cfg(target_os = "macos")]
pub(crate) fn apply(window: &dyn iced::window::Window) -> bool {
    use window_vibrancy::{LiquidGlassOptions, NSGlassEffectViewStyle, apply_liquid_glass};

    let options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Clear);
    apply_liquid_glass(window, options).is_ok()
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn apply(_window: &dyn iced::window::Window) -> bool {
    false
}
