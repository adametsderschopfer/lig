#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{center, text};
use iced::{Color, Element, Program, Size, Theme, theme, window};

fn main() -> iced::Result {
    application().run()
}

fn application() -> iced::Application<impl Program<State = Lig, Message = Message, Theme = Theme>> {
    iced::application(Lig::default, Lig::update, Lig::view)
        .title("lig — Life is Graph")
        .window(window::Settings {
            size: Size::new(960.0, 640.0),
            min_size: Some(Size::new(320.0, 240.0)),
            position: window::Position::Centered,
            transparent: true,
            ..window::Settings::default()
        })
        .style(|_state, theme| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        })
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
        center(text("Hello, world!").size(40)).into()
    }
}

#[cfg(test)]
mod tests;
