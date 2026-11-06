#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{center, container, text};
use iced::{Border, Color, Degrees, Element, Program, Shadow, Size, Theme, Vector};
use iced::{gradient, theme, window};

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
            transparent: false,
            ..window::Settings::default()
        })
        .theme(Theme::Light)
        .style(|_state, _theme| theme::Style {
            background_color: Color::from_rgb8(220, 228, 239),
            text_color: Color::from_rgb8(39, 49, 65),
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
        let surface = center(text("Hello, world!").size(36))
            .padding(12)
            .style(glass_surface);

        container(surface).padding(16).into()
    }
}

fn glass_surface(_theme: &Theme) -> container::Style {
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
mod tests;
