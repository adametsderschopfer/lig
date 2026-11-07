use super::*;
use crate::i18n::Language;

fn english_application()
-> iced::Application<impl Program<State = Lig, Message = Message, Theme = Theme>> {
    application(Rc::new(I18n::new(Language::English)))
}

#[test]
fn application_boots_into_the_welcome_screen() {
    let app = english_application();
    let (state, _) = app.boot();
    let settings = Program::window(&app).expect("The application must open a window");

    assert_eq!(settings.size, Size::new(960.0, 640.0));
    assert_eq!(settings.min_size, Some(Size::new(320.0, 240.0)));

    let mut ui =
        iced_test::Simulator::with_size(iced::Settings::default(), settings.size, state.view());
    let greeting = ui
        .find("Hello, world!")
        .expect("The greeting must be present");

    assert!(greeting.visible_bounds().is_some());
}

#[test]
fn native_window_allows_transparency_only_on_macos() {
    let app = english_application();
    let settings = Program::window(&app).expect("The application must open a window");

    assert_eq!(settings.transparent, cfg!(target_os = "macos"));
    assert!(settings.decorations);
    #[cfg(target_os = "macos")]
    {
        assert!(settings.platform_specific.title_hidden);
        assert!(settings.platform_specific.titlebar_transparent);
        assert!(settings.platform_specific.fullsize_content_view);
    }
}

#[test]
fn background_stays_opaque_until_native_glass_is_ready() {
    let app = english_application();
    let (state, _) = app.boot();

    for theme in Theme::ALL {
        let style = Program::style(&app, &state, theme);

        assert_eq!(style.background_color.a, 1.0, "Theme: {theme:?}");
        assert_eq!(style.text_color.a, 1.0, "Text must remain opaque");
    }
}

#[test]
fn native_glass_result_controls_transparency() {
    let app = english_application();
    let (mut state, _) = app.boot();

    for (ready, alpha) in [(false, 1.0), (true, 0.0), (false, 1.0)] {
        let _ = state.update(Message::GlassConfigured(ready));
        let style = Program::style(&app, &state, &Theme::Dark);

        assert_eq!(style.background_color.a, alpha);
        assert_eq!(style.text_color.a, 1.0);
        let mut ui = iced_test::Simulator::with_size(
            iced::Settings::default(),
            (320.0, 240.0),
            state.view(),
        );
        assert!(ui.find("Hello, world!").unwrap().visible_bounds().is_some());
    }
}

#[test]
fn window_title_contains_the_product_name() {
    let app = english_application();
    let (state, _) = app.boot();

    assert_eq!(
        Program::title(&app, &state, window::Id::unique()),
        "lig — Life is Graph"
    );
}

#[test]
fn title_and_initial_screen_use_the_selected_language() {
    for (language, title, greeting) in [
        (Language::English, "lig — Life is Graph", "Hello, world!"),
        (Language::Russian, "lig — Жизнь — это граф", "Привет, мир!"),
        (
            Language::Spanish,
            "lig — La vida es un grafo",
            "¡Hola, mundo!",
        ),
    ] {
        let app = application(Rc::new(I18n::new(language)));
        let (state, _) = app.boot();
        assert_eq!(Program::title(&app, &state, window::Id::unique()), title);
        let mut ui = iced_test::Simulator::with_size(
            iced::Settings::default(),
            (320.0, 240.0),
            state.view(),
        );
        assert!(ui.find(greeting).unwrap().visible_bounds().is_some());
    }
}
