use super::*;

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
