use crate::system::SystemSnapshot;
use crate::ui::theme::Theme;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn gauge<'a>(label: &'a str, value: f64, t: Theme) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().borders(Borders::NONE).title(label))
        .gauge_style(Style::default().fg(t.accent))
        .ratio((value / 100.0).clamp(0.0, 1.0))
}
pub fn info_lines<'a>(s: &'a SystemSnapshot, t: Theme) -> Vec<Line<'a>> {
    vec![
        Line::from(vec![
            Span::styled("OS        ", Style::default().fg(t.accent)),
            Span::raw(format!("{} — {}", s.os.name, s.os.version)),
        ]),
        Line::from(format!("Kernel    {}", s.kernel)),
        Line::from(format!("Host      {}", s.hostname)),
        Line::from(format!("Arch      {}", s.architecture)),
        Line::from(format!(
            "Uptime    {}",
            crate::system::uptime::format_uptime(std::time::Duration::from_secs(s.uptime_seconds))
        )),
        Line::from(""),
        Line::from(format!("CPU       {}", s.cpu.model)),
        Line::from(format!("CPU       {:.0}%", s.cpu.usage_percent)),
        Line::from(format!(
            "Memory    {} / {}",
            crate::system::memory::format_bytes(s.memory.used_bytes),
            crate::system::memory::format_bytes(s.memory.total_bytes)
        )),
        Line::from(format!(
            "Disk      {} / {}",
            crate::system::memory::format_bytes(s.disk.used_bytes),
            crate::system::memory::format_bytes(s.disk.total_bytes)
        )),
        Line::from(format!("Shell     {}", s.shell.name)),
        Line::from(format!("Terminal  {}", s.terminal.name)),
        Line::from(format!("DE        {}", s.desktop.desktop)),
        Line::from(format!("Display   {}", s.desktop.display)),
    ]
}
pub fn info_paragraph<'a>(s: &'a SystemSnapshot, t: Theme) -> Paragraph<'a> {
    Paragraph::new(info_lines(s, t)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.border))
            .title(Span::styled(
                " SYSTEM INFORMATION ",
                crate::ui::theme::title(t),
            )),
    )
}
