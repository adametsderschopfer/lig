//! Render the initial greeting without application or infrastructure dependencies.

use crate::i18n::{I18n, Key};
use crate::ui::theme;
use iced::Element;
use iced::widget::{center, text};

pub(crate) fn view<'a, Message: 'a>(i18n: &I18n) -> Element<'a, Message> {
    center(text(i18n.text(Key::WelcomeGreeting)).size(theme::GREETING_SIZE)).into()
}

#[cfg(test)]
#[path = "../../../tests/unit/welcome.rs"]
mod tests;
