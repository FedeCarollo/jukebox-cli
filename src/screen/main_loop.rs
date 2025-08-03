use std::io;
use std::path::Path;

use crossterm::event;
use ratatui::{layout::{Constraint, Direction, Layout}, prelude::Backend, widgets::{Block, Borders}, Terminal};

use crate::{jukebox_state, screen::jukebox_side::{render_jukebox_matrix, JukeboxMatrixState}};
use super::playlist_side::render_playlist_side;
use super::controls_block::render_controls_block;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.clear()?;
    terminal.hide_cursor()?;
    // Usa una directory di esempio - puoi cambiarla
    let music_path = Path::new(".");
    let mut jukebox_state = jukebox_state::JukeboxState::new(music_path);
    let jukebox_matrix_state = JukeboxMatrixState::new();
    loop {
        terminal.draw(|f| {
            let size = f.area();
            // Dividi lo schermo in verticale: blocco superiore 80%, inferiore 20%
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ].as_ref())
                .split(size);

            // Nel blocco superiore, dividi in due blocchi orizzontali
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(vertical_chunks[0]);

            let block_left = Block::default().title("Left").borders(Borders::ALL);
            f.render_widget(block_left, top_chunks[0]);
            render_playlist_side(f, top_chunks[1], &jukebox_state);
            render_jukebox_matrix(f, top_chunks[0], &jukebox_matrix_state);

            // Blocco inferiore con i controlli
            render_controls_block(f, vertical_chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('p') => jukebox_state.play(),
                        event::KeyCode::Char('s') => jukebox_state.pause(),
                        event::KeyCode::Char('+') => jukebox_state.add_volume(10),
                        event::KeyCode::Char('-') => jukebox_state.sub_volume(10),
                        event::KeyCode::Down => jukebox_state.move_selection(1),
                        event::KeyCode::Up => jukebox_state.move_selection(-1),
                        event::KeyCode::Enter => jukebox_state.play(),
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}