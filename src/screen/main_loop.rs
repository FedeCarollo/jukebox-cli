use std::io;
use std::path::Path;

use crossterm::event;
use ratatui::{prelude::Backend, Terminal};

use crate::{jukebox_state, screen::{block_utils::{make_horizontal_chunks, make_vertical_chunks}, jukebox_side::{render_jukebox_matrix}}};
use super::playlist_side::render_playlist_side;
use super::controls_block::render_controls_block;
use super::info_block::render_info_block;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.clear()?;
    terminal.hide_cursor()?;
    // Usa una directory di esempio - puoi cambiarla
    let music_path = Path::new(".");
    let mut jukebox_state = jukebox_state::JukeboxState::new(music_path);
    loop {
        terminal.draw(|f| {
            let size = f.area();
            // Dividi lo schermo in verticale: blocco superiore 80%, inferiore 20%
            let vertical_chunks = make_vertical_chunks(size, &[80, 20]);

            // Nel blocco superiore, dividi in due blocchi orizzontali
            let top_chunks = make_horizontal_chunks(vertical_chunks[0], &[70, 30]);

            let jukebox_info_chunks = make_vertical_chunks(top_chunks[0], &[85, 15]);

            let jukebox_block = jukebox_info_chunks[0]; //Show jukebox
            let info_block = jukebox_info_chunks[1];    //Show volume level and song progress

            let song_block = top_chunks[1];
            let controls_block = vertical_chunks[1];

            render_info_block(f, info_block, &jukebox_state);
            render_playlist_side(f, song_block, &jukebox_state);
            render_jukebox_matrix(f, jukebox_block, &jukebox_state);

            // Blocco inferiore con i controlli
            render_controls_block(f, controls_block);
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