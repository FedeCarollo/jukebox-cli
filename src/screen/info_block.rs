use crate::{jukebox_state::JukeboxState, screen::block_utils::make_horizontal_chunks};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
};
use std::time::Duration;

/// Format duration into a string "MM:SS"
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// Get emoji based on volume level
fn get_volume_emoji(volume: u8) -> &'static str {
    match volume {
        0 => "🔇",
        1..=33 => "🔈",
        34..=66 => "🔉",
        67..=100 => "🔊",
        _ => "🔊",
    }
}

/// Disegna il blocco delle informazioni con progress bar e volume
pub fn render_info_block(f: &mut Frame, area: Rect, jukebox_state: &JukeboxState) {
    // Split area into two parts: progress (70%) and volume (30%)
    let chunks = make_horizontal_chunks(area, &[70, 30]);

    render_progress_bar(f, chunks[0], jukebox_state);
    
    render_volume_bar(f, chunks[1], jukebox_state);
}

fn render_progress_bar(f: &mut Frame, area: Rect, jukebox_state: &JukeboxState) {
    if let Some(playing_song) = jukebox_state.currently_playing() {
        let current_pos = jukebox_state.current_playback_position();
        let total_duration = playing_song.duration().unwrap_or(Duration::from_secs(0));
        let progress_ratio = jukebox_state.progress_ratio();
        
        let current_time = format_duration(current_pos);
        let total_time = format_duration(total_duration);
        
        // Progress bar for song duration
        let progress_text = format!("{} / {}", current_time, total_time);
        
        let progress_bar = Gauge::default()
            .block(Block::default().title("Progress").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .label(progress_text)
            .ratio(progress_ratio as f64);
        f.render_widget(progress_bar, area);
    } else {
        // No song playing, show a default message
        let no_progress_bar = Gauge::default()
            .block(Block::default().title("Progress").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Gray))
            .label("No song playing")
            .ratio(0.0);
        f.render_widget(no_progress_bar, area);
    }
}

fn render_volume_bar(f: &mut Frame, area: Rect, jukebox_state: &JukeboxState) {
    let volume = jukebox_state.volume();
    let volume_emoji = get_volume_emoji(volume);
    let volume_ratio = volume as f64 / 100.0;
    
    // Progress bar for volume
    let volume_text = format!("{} {}%", volume_emoji, volume);
    
    let volume_bar = Gauge::default()
        .block(Block::default().title("Volume").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .label(volume_text)
        .ratio(volume_ratio);
    f.render_widget(volume_bar, area);
}
