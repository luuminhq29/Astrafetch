use crate::ui::{layout, logo, theme, widgets};
use crate::{config::Config, system::SystemSnapshot};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, s: &SystemSnapshot, c: &Config, frame_no: u64, paused: bool) {
    let t = theme::get(&c.theme);
    let (left, right) = layout::split_main(frame.area());
    let logo_lines = logo::render(&c.logo, &s.os);
    let logo_text = logo_lines.join("\n");

    let pulse = if c.animation && !paused {
        crate::ui::animation::breathing(frame_no)
    } else {
        1.0
    };

    let logo_style = Style::default().fg(t.accent).add_modifier(if pulse > 0.6 {
        Modifier::BOLD
    } else {
        Modifier::DIM
    });

    let lp = Paragraph::new(logo_text)
        .style(logo_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .title(" OS LOGO "),
        );

    frame.render_widget(lp, left);
    frame.render_widget(widgets::info_paragraph(s, t), right);

    let footer = if c.author {
        format!(
            "AstraFetch  •  by Lưu Minh Quang - Astra  •  {}",
            if paused { "PAUSED" } else { "LIVE" }
        )
    } else {
        "AstraFetch".into()
    };

    let area = Rect {
        x: 0,
        y: frame.area().height.saturating_sub(1),
        width: frame.area().width,
        height: 1,
    };

    frame.render_widget(
        Paragraph::new(footer)
            .style(Style::default().fg(t.muted))
            .alignment(Alignment::Center),
        area,
    );
}
