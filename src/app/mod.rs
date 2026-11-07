//! Compose application state, messages, and window settings.

use crate::shared::i18n::{I18n, Key};
use crate::{features::welcome, shared::theme};
use iced::{Element, Program, Size, Theme, window};
use std::rc::Rc;

pub(crate) fn run(i18n: Rc<I18n>) -> iced::Result {
    application(i18n).run()
}

fn application(
    i18n: Rc<I18n>,
) -> iced::Application<impl Program<State = Lig, Message = Message, Theme = Theme>> {
    iced::application(
        move || Lig {
            i18n: Rc::clone(&i18n),
        },
        Lig::update,
        Lig::view,
    )
    .title(Lig::title)
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

struct Lig {
    i18n: Rc<I18n>,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Lig {
    fn title(&self) -> String {
        self.i18n.text(Key::AppTitle)
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        welcome::view(&self.i18n)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/app.rs"]
mod tests;
