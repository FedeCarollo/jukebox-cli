use ratatui::{
    layout::Rect, text::{Line}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::{canvas_state::CanvasState, jukebox_state::JukeboxState};

/// Disegna la matrice di caratteri del jukebox
pub fn render_jukebox_matrix(f: &mut Frame, area: Rect, state: &mut CanvasState, jukebox_state: &JukeboxState) {
    // Update the canvas state with the current jukebox state
    state.update_notes(area.width, area.height, jukebox_state.is_playing());
    state.update_is_playing(jukebox_state.is_playing());

    let canvas = state.get_canvas(area.width, area.height);

    let mut lines: Vec<Line<'static>> = Vec::new();
    for row in canvas {
        let line = Line::from(row);
        lines.push(line);
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::RIGHT))
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, area);

    
}