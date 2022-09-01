use iced::{button, Background, Color, Vector};

pub enum Button {
    Primary,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb(0.3, 1.0, 0.5),
                Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
            })),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::BLACK,
            ..button::Style::default()
        }
    }
}
