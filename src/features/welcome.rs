//! Render the initial greeting without application or infrastructure dependencies.

use crate::shared::theme;
use iced::Element;
use iced::widget::{center, container, text};

pub(crate) fn view<'a, Message: 'a>() -> Element<'a, Message> {
    let surface = center(text("Hello, world!").size(theme::GREETING_SIZE))
        .padding(theme::SURFACE_PADDING)
        .style(theme::glass_surface);

    container(surface).padding(theme::WINDOW_PADDING).into()
}

#[cfg(test)]
#[path = "../../tests/unit/welcome.rs"]
mod tests;
