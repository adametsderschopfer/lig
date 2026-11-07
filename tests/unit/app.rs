use super::*;
use iced_test::Simulator;

#[test]
fn hello_world_is_visible_and_centered_at_supported_window_sizes() {
    let state = Lig;

    for size in [(320.0, 240.0), (960.0, 640.0), (1920.0, 1080.0)] {
        let mut ui = Simulator::with_size(iced::Settings::default(), size, state.view());

        let greeting = ui
            .find("Hello, world!")
            .expect("The greeting must be present");
        let bounds = greeting.bounds();

        let visible = greeting
            .visible_bounds()
            .expect("The greeting must be visible");

        // Clipping recomputes floating-point bounds, so allow subpixel rounding.
        for (actual, expected) in [
            (visible.x, bounds.x),
            (visible.y, bounds.y),
            (visible.width, bounds.width),
            (visible.height, bounds.height),
        ] {
            assert!((actual - expected).abs() < 0.001);
        }
        assert!(bounds.width > 0.0 && bounds.height > 0.0);
        assert!((bounds.center_x() - size.0 / 2.0).abs() <= 1.0);
        assert!((bounds.center_y() - size.1 / 2.0).abs() <= 1.0);
    }
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

#[test]
fn glass_surface_does_not_expose_the_desktop() {
    let style = glass_surface(&Theme::Light);
    let Some(iced::Background::Gradient(iced::Gradient::Linear(gradient))) = style.background
    else {
        panic!("The glass surface must have a gradient background");
    };

    let stops: Vec<_> = gradient.stops.iter().flatten().collect();
    assert!(stops.len() >= 2);
    assert!(stops.iter().all(|stop| stop.color.a == 1.0));
}
