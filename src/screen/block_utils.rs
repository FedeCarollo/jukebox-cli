use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn make_vertical_chunks(area: Rect, proportions: &[u16]) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(proportions.iter().map(|&p| Constraint::Percentage(p)).collect::<Vec<_>>())
        .split(area)
        .to_vec()
}

pub fn make_horizontal_chunks(area: Rect, proportions: &[u16]) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(proportions.iter().map(|&p| Constraint::Percentage(p)).collect::<Vec<_>>())
        .split(area)
        .to_vec()
}

#[allow(dead_code)]
pub fn make_chunks(area: Rect, direction: Direction, constraints: &[Constraint]) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints.to_vec())
        .split(area)
        .to_vec()
}