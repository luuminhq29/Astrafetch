use ratatui::prelude::*;
pub fn split_main(area: Rect) -> (Rect, Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(31), Constraint::Percentage(69)])
        .split(area);
    (cols[0], cols[1])
}
pub fn split_dashboard(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(5),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(area)
        .to_vec()
}
