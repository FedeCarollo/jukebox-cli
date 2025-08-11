use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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
    game,
};

enum AppMode {
    Jukebox,
    DrivingGame,
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    music_path: Option<PathBuf>,
) -> io::Result<()> {
    terminal.clear()?;
    terminal.hide_cursor()?;

    let music_path = music_path.unwrap_or_else(|| Path::new("example_music").to_path_buf());
    let mut jukebox_state = jukebox_state::JukeboxState::new(&music_path);
    let mut canvas_state = canvas_state::CanvasState::new();

    let mut mode = AppMode::Jukebox;
    let mut game_state: Option<game::state::GameState> = None;
    let mut music_during_game = true;

    let fixed_step = Duration::from_millis(33); // ~30 FPS
    let mut accumulator = Duration::ZERO;
    let mut last = Instant::now();

    loop {
        let now = Instant::now();
        let frame_dt = now.saturating_duration_since(last);
        last = now;
        accumulator += frame_dt;

        // Drain input events quickly
        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                event::Event::Key(key) if key.kind == event::KeyEventKind::Press => {
                    match mode {
                        AppMode::Jukebox => {
                            match key.code {
                                event::KeyCode::Char('q') => break,
                                event::KeyCode::Char('p') | event::KeyCode::Enter => jukebox_state.play(),
                                event::KeyCode::Char('s') => jukebox_state.pause(),
                                event::KeyCode::Char('+') => jukebox_state.add_volume(10),
                                event::KeyCode::Char('-') => jukebox_state.sub_volume(10),
                                event::KeyCode::Down => jukebox_state.move_selection(1),
                                event::KeyCode::Up => jukebox_state.move_selection(-1),
                                event::KeyCode::Char('g') => {
                                    // start game
                                    let sz = terminal.size()?;
                                    let rect = ratatui::layout::Rect::new(0, 0, sz.width, sz.height);
                                    let seed = now.elapsed().as_nanos() as u64
                                        ^ now.elapsed().as_micros() as u64
                                        ^ 0x9E3779B97F4A7C15;
                                    game_state = Some(game::state::GameState::new(rect, seed));
                                    mode = AppMode::DrivingGame;
                                }
                                _ => {}
                            }
                        }
                        AppMode::DrivingGame => {
                            if let Some(cmd) = game::input::map_key(key) {
                                if let Some(gs) = &mut game_state {
                                    use game::input::GameCmd::*;
                                    use game::scene::GameScene;
                                    match cmd {
                                        MoveLeft => {
                                            if gs.player.lane > 0 { gs.player.lane -= 1; }
                                            gs.player.x = gs.lane_x(gs.player.lane);
                                        }
                                        MoveRight => {
                                            if gs.player.lane < 2 { gs.player.lane += 1; }
                                            gs.player.x = gs.lane_x(gs.player.lane);
                                        }
                                        MoveUp => {
                                            gs.player.y = gs.player.y.saturating_sub(1).max(1);
                                        }
                                        MoveDown => {
                                            gs.player.y = (gs.player.y + 1).min(gs.height.saturating_sub(2));
                                        }
                                        Fire => {
                                            let now = Instant::now();
                                            if now >= gs.player.fire_cooldown_until {
                                                gs.player.fire_cooldown_until = now + Duration::from_millis(150);
                                                gs.projectiles.push(game::entities::Projectile {
                                                    x: gs.player.x,
                                                    y: gs.player.y.saturating_sub(1) as f32,
                                                    speed: 40.0,
                                                });
                                            }
                                        }
                                        Pause => {
                                            gs.scene = match gs.scene {
                                                GameScene::Running => GameScene::Paused,
                                                GameScene::Paused => GameScene::Running,
                                                s => s,
                                            };
                                        }
                                        Restart => {
                                            let sz = terminal.size()?;
                                            let rect = ratatui::layout::Rect::new(0, 0, sz.width, sz.height);
                                            let seed = now.elapsed().as_nanos() as u64 ^ 0xA2B79C3D;
                                            *gs = game::state::GameState::new(rect, seed);
                                        }
                                        ToggleMusic => {
                                            music_during_game = !music_during_game;
                                            if !music_during_game {
                                                jukebox_state.pause();
                                            }
                                        }
                                        Exit => {
                                            mode = AppMode::Jukebox;
                                            game_state = None;
                                        }
                                    }
                                } else {
                                    // no state -> exit to jukebox
                                    mode = AppMode::Jukebox;
                                }
                            }
                        }
                    }
                }
                event::Event::Resize(w, h) => {
                    if let Some(gs) = &mut game_state {
                        let rect = ratatui::layout::Rect::new(0, 0, w, h);
                        gs.resize(rect);
                    }
                }
                _ => {}
            }
        }

        // Update audio progression (independent)
        jukebox_state.handle_song_end();

        // Update game with fixed timestep
        if let (AppMode::DrivingGame, Some(gs)) = (&mode, &mut game_state) {
            while accumulator >= fixed_step {
                game::update::update(gs, fixed_step);
                accumulator -= fixed_step;
            }
        } else {
            // In jukebox mode we can throttle accumulator
            if accumulator >= Duration::from_millis(100) {
                accumulator = Duration::ZERO;
            }
        }

        terminal.draw(|f| {
            let size = f.area();

            let vertical_chunks = make_vertical_chunks(size, &[80, 20]);

            let top_chunks = make_horizontal_chunks(vertical_chunks[0], &[70, 30]);

            let jukebox_chunk = top_chunks[0]; // main left area
            let controls_info_chunk = make_horizontal_chunks(vertical_chunks[1], &[50, 50]);
            let controls_chunk = controls_info_chunk[0]; // controls
            let info_chunk = controls_info_chunk[1]; // info
            let song_chunk = top_chunks[1]; // playlist side

            render_info_block(f, info_chunk, &jukebox_state);
            render_playlist_side(f, song_chunk, &jukebox_state);

            match mode {
                AppMode::Jukebox => {
                    canvas_state.update_is_playing(jukebox_state.is_playing());
                    canvas_state.update_notes(jukebox_chunk.width, jukebox_chunk.height, jukebox_state.is_playing());
                    render_jukebox_matrix(f, jukebox_chunk, &mut canvas_state, &jukebox_state);
                }
                AppMode::DrivingGame => {
                    if let Some(gs) = &mut game_state {
                        // Ensure state matches this specific area
                        if gs.width != jukebox_chunk.width || gs.height != jukebox_chunk.height {
                            gs.resize(jukebox_chunk);
                        }
                        game::draw::draw(f, gs, jukebox_chunk);
                    }
                }
            }

            render_controls_block(f, controls_chunk);
        })?;
    }
    Ok(())
}
