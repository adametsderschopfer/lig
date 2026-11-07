//! Compose application state, messages, and window settings.

use crate::{features::welcome, shared::theme};
use iced::{Element, Program, Size, Theme, window};

pub(crate) fn run() -> iced::Result {
    application().run()
}

fn application() -> iced::Application<impl Program<State = Lig, Message = Message, Theme = Theme>> {
    iced::application(Lig::default, Lig::update, Lig::view)
        .title("lig — Life is Graph")
        .window(window::Settings {
            size: Size::new(960.0, 640.0),
            min_size: Some(Size::new(320.0, 240.0)),
            position: window::Position::Centered,
            transparent: false,
            ..window::Settings::default()
        })
        .theme(Theme::Light)
        .style(|_, _| theme::application_style())
}

#[derive(Default)]
struct Lig;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Lig {
    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        welcome::view()
    }
}

#[cfg(test)]
#[path = "../../tests/unit/app.rs"]
mod tests;
