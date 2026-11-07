use super::*;
use iced_test::Simulator;

#[test]
fn hello_world_is_visible_and_centered_at_supported_window_sizes() {
    for size in [(320.0, 240.0), (960.0, 640.0), (1920.0, 1080.0)] {
        let mut ui = Simulator::with_size(iced::Settings::default(), size, view::<()>());

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
