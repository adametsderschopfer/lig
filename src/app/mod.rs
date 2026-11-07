//! Compose application state, messages, and window settings.

use crate::i18n::{I18n, Key};
use crate::infrastructure::window_effect;
use crate::ui::{screens::welcome, theme};
use iced::{Element, Program, Size, Task, Theme, window};
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
            glass_ready: false,
        },
        Lig::update,
        Lig::view,
    )
    .title(Lig::title)
    .window(window::Settings {
        size: Size::new(960.0, 640.0),
        min_size: Some(Size::new(320.0, 240.0)),
        position: window::Position::Centered,
        transparent: cfg!(target_os = "macos"),
        #[cfg(target_os = "macos")]
        platform_specific: window::settings::PlatformSpecific {
            title_hidden: true,
            titlebar_transparent: true,
            fullsize_content_view: true,
        },
        ..window::Settings::default()
    })
    .subscription(|_| window::open_events().map(Message::WindowOpened))
    .theme(Theme::Dark)
    .style(|state, _| theme::application_style(state.glass_ready))
}

struct Lig {
    i18n: Rc<I18n>,
    glass_ready: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    WindowOpened(window::Id),
    GlassConfigured(bool),
}

impl Lig {
    fn title(&self) -> String {
        self.i18n.text(Key::AppTitle)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => {
                window::run(id, window_effect::apply).map(Message::GlassConfigured)
            }
            Message::GlassConfigured(ready) => {
                self.glass_ready = ready;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        welcome::view(&self.i18n)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/app.rs"]
mod tests;
