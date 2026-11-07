use super::*;

#[test]
fn application_boots_into_the_welcome_screen() {
    let app = application();
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
fn native_window_is_opaque() {
    let app = application();
    let settings = Program::window(&app).expect("The application must open a window");

    assert!(!settings.transparent);
}

#[test]
fn background_stays_opaque_in_every_theme() {
    let app = application();
    let (state, _) = app.boot();

    for theme in Theme::ALL {
        let style = Program::style(&app, &state, theme);

        assert_eq!(style.background_color.a, 1.0, "Theme: {theme:?}");
        assert_eq!(style.text_color.a, 1.0, "Text must remain opaque");
    }
}

#[test]
fn window_title_contains_the_product_name() {
    let app = application();
    let (state, _) = app.boot();

    assert_eq!(
        Program::title(&app, &state, window::Id::unique()),
        "lig — Life is Graph"
    );
}
