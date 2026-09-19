use ratatui::prelude::*;

pub fn split_main(area: Rect) -> (Rect, Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(31), Constraint::Percentage(69)])
        .split(area);
    (cols[0], cols[1])
}
