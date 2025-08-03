use crate::jukebox_state::JukeboxState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

const PALETTE: [Color;13] = [
        Color::Blue,
        Color::Green,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::LightBlue,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightMagenta,
        Color::LightCyan,
        Color::Gray,
        Color::Red,
        Color::LightRed,
    ];

fn get_song_list(jukebox_state: &JukeboxState) -> Vec<ListItem> {
    
    let selected = jukebox_state.current_selection();
    jukebox_state
        .playlist()
        .iter()
        .enumerate()
        .map(|(i, song)| {
            let song_name = song.title().to_string();
            let mut style = Style::default().fg(PALETTE[i % PALETTE.len()]);
            
            if selected == song {
                style = style.add_modifier(ratatui::style::Modifier::UNDERLINED);
            }
            
            ListItem::new(song_name).style(style)
        })
        .collect()
}

/// Disegna il lato playlist (destro) con la lista delle canzoni disponibili
pub fn render_playlist_side(f: &mut Frame, area: Rect, jukebox_state: &JukeboxState) {
    let songs: Vec<ListItem> = get_song_list(jukebox_state);
    let songs_list = List::new(songs).block(
        Block::default()
            .title("Available Songs")
            .borders(Borders::NONE),
    );
    f.render_widget(songs_list, area);
}
