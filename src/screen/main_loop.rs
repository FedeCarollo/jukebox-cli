use std::io;
use std::path::{Path, PathBuf};

use crossterm::event;
use ratatui::{Terminal, prelude::Backend};

use super::controls_block::render_controls_block;
use super::info_block::render_info_block;
use super::playlist_side::render_playlist_side;
use crate::{
    canvas_state, jukebox_state,
    screen::{
        block_utils::{make_horizontal_chunks, make_vertical_chunks},
        jukebox_side::render_jukebox_matrix,
    },
};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    music_path: Option<PathBuf>,
) -> io::Result<()> {
    terminal.clear()?;
    terminal.hide_cursor()?;

    let music_path = music_path.unwrap_or_else(|| Path::new("example_music").to_path_buf());
    let mut jukebox_state = jukebox_state::JukeboxState::new(&music_path);
    let mut canvas_state = canvas_state::CanvasState::new();
    loop {
        terminal.draw(|f| {
            let size = f.area();

            let vertical_chunks = make_vertical_chunks(size, &[80, 20]);

            let top_chunks = make_horizontal_chunks(vertical_chunks[0], &[70, 30]);

            let jukebox_chunk = top_chunks[0]; // Show jukebox matrix
            let controls_info_chunk = make_horizontal_chunks(vertical_chunks[1], &[50, 50]);
            let controls_chunk = controls_info_chunk[0]; // Show controls
            let info_chunk = controls_info_chunk[1]; // Show info block
            let song_chunk = top_chunks[1]; // Show playlist side

            render_info_block(f, info_chunk, &jukebox_state);
            render_playlist_side(f, song_chunk, &jukebox_state);
            render_jukebox_matrix(f, jukebox_chunk, &mut canvas_state, &jukebox_state);
            render_controls_block(f, controls_chunk);
        })?;

        // Check if the song has ended
        jukebox_state.handle_song_end();

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
