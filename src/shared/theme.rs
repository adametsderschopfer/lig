//! Define the current light theme and opaque glass surface.

use iced::widget::container;
use iced::{Border, Color, Degrees, Shadow, Theme, Vector, gradient, theme};

pub(crate) const GREETING_SIZE: u32 = 36;
pub(crate) const SURFACE_PADDING: u16 = 12;
pub(crate) const WINDOW_PADDING: u16 = 16;

pub(crate) fn application_style() -> theme::Style {
    theme::Style {
        background_color: Color::from_rgb8(220, 228, 239),
        text_color: Color::from_rgb8(39, 49, 65),
    }
}

pub(crate) fn glass_surface(_theme: &Theme) -> container::Style {
    let background = gradient::Linear::new(Degrees(135.0))
        .add_stop(0.0, Color::from_rgb8(255, 255, 255))
        .add_stop(0.22, Color::from_rgb8(239, 247, 252))
        .add_stop(0.48, Color::from_rgb8(222, 234, 247))
        .add_stop(0.74, Color::from_rgb8(238, 233, 246))
        .add_stop(1.0, Color::from_rgb8(251, 250, 253));

    container::Style {
        background: Some(background.into()),
        border: Border {
            color: Color::WHITE,
            width: 1.0,
            radius: 24.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba8(69, 86, 116, 0.16),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

#[cfg(test)]
#[path = "../../tests/unit/theme.rs"]
mod tests;
