use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Color,
    pub muted: Color,
    pub border: Color,
}

pub fn get(name: &str) -> Theme {
    match name {
        "cyber" => Theme {
            accent: Color::Rgb(0, 255, 198),
            muted: Color::DarkGray,
            border: Color::Rgb(0, 180, 180),
        },
        "aurora" => Theme {
            accent: Color::Cyan,
            muted: Color::Gray,
            border: Color::Cyan,
        },
        "matrix" => Theme {
            accent: Color::Green,
            muted: Color::DarkGray,
            border: Color::Green,
        },
        "minimal" => Theme {
            accent: Color::White,
            muted: Color::Gray,
            border: Color::Gray,
        },
        "monochrome" => Theme {
            accent: Color::White,
            muted: Color::Gray,
            border: Color::White,
        },
        _ => Theme {
            accent: Color::Cyan,
            muted: Color::Gray,
            border: Color::Cyan,
        },
    }
}

pub fn title(t: Theme) -> Style {
    Style::default().fg(t.accent).add_modifier(Modifier::BOLD)
}
