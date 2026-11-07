use super::*;

#[test]
fn fallback_is_opaque_neutral_gray() {
    let style = application_style(false);
    let background = style.background_color;

    assert_eq!(background, Color::from_rgb8(24, 24, 24));
    assert_eq!(background.a, 1.0);
    assert_eq!(background.r, background.g);
    assert_eq!(background.g, background.b);
    assert!(style.text_color.r > background.r);
    assert_eq!(style.text_color.a, 1.0);
}

#[test]
fn native_glass_keeps_the_background_clear_and_text_opaque() {
    let style = application_style(true);

    assert_eq!(style.background_color, Color::TRANSPARENT);
    assert_eq!(style.text_color.a, 1.0);
}
