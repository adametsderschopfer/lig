//! Render the initial greeting without application or infrastructure dependencies.

use crate::shared::i18n::{I18n, Key};
use crate::shared::theme;
use iced::Element;
use iced::widget::{center, container, text};

pub(crate) fn view<'a, Message: 'a>(i18n: &I18n) -> Element<'a, Message> {
    let surface = center(text(i18n.text(Key::WelcomeGreeting)).size(theme::GREETING_SIZE))
        .padding(theme::SURFACE_PADDING)
        .style(theme::glass_surface);

    container(surface).padding(theme::WINDOW_PADDING).into()
}

#[cfg(test)]
#[path = "../../tests/unit/welcome.rs"]
mod tests;
