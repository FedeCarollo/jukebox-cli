use ratatui::{prelude::*};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use std::{io, error::Error, path::PathBuf};
use clap::Parser;

use crate::screen::main_loop::run_app;

mod jukebox_state;
mod canvas_state;
mod screen;

#[derive(Parser)]
#[command(name = "jukebox-cli")]
#[command(about = "A terminal-based music jukebox application")]
struct Args {
    /// Path to the music directory
    #[arg(help = "Directory containing MP3 files. If not provided, defaults to a sample directory.")]
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let music_path = args.path;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, music_path);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
    Ok(())
}


