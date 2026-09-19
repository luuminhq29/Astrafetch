use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Color,
    pub accent2: Color,
    pub text: Color,
    pub muted: Color,
    pub border: Color,
    pub good: Color,
}
pub fn get(name: &str) -> Theme {
    match name {
        "cyber" => Theme {
            accent: Color::Rgb(0, 255, 198),
            accent2: Color::Rgb(0, 170, 255),
            text: Color::White,
            muted: Color::DarkGray,
            border: Color::Rgb(0, 180, 180),
            good: Color::Green,
        },
        "aurora" => Theme {
            accent: Color::Cyan,
            accent2: Color::Magenta,
            text: Color::White,
            muted: Color::Gray,
            border: Color::Cyan,
            good: Color::Green,
        },
        "matrix" => Theme {
            accent: Color::Green,
            accent2: Color::DarkGray,
            text: Color::Green,
            muted: Color::DarkGray,
            border: Color::Green,
            good: Color::Green,
        },
        "minimal" => Theme {
            accent: Color::White,
            accent2: Color::Gray,
            text: Color::White,
            muted: Color::Gray,
            border: Color::Gray,
            good: Color::White,
        },
        "monochrome" => Theme {
            accent: Color::White,
            accent2: Color::White,
            text: Color::White,
            muted: Color::Gray,
            border: Color::White,
            good: Color::White,
        },
        _ => Theme {
            accent: Color::Cyan,
            accent2: Color::Blue,
            text: Color::White,
            muted: Color::Gray,
            border: Color::Cyan,
            good: Color::Green,
        },
    }
}
pub fn title(t: Theme) -> Style {
    Style::default().fg(t.accent).add_modifier(Modifier::BOLD)
}
