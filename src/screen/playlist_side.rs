use crate::jukebox_state::JukeboxState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
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
    let playing = jukebox_state.currently_playing();
    jukebox_state
        .playlist()
        .iter()
        .enumerate()
        .map(|(i, song)| {
            let song_name = song.title().split('.').next().unwrap_or("").to_string();
            let mut style = Style::default().fg(PALETTE[i % PALETTE.len()]);
            
            if selected == song {
                style = style.add_modifier(ratatui::style::Modifier::UNDERLINED);
            }

            if Some(song) == playing {
                style = style.add_modifier(ratatui::style::Modifier::ITALIC);
                style = style.bg(PALETTE[(i + 1) % PALETTE.len()]);
            }
            
            ListItem::new(song_name).style(style)
        })
        .collect()
}

/// Disegna il lato playlist (destro) con la lista delle canzoni disponibili
pub fn render_playlist_side(f: &mut Frame, area: Rect, jukebox_state: &JukeboxState) {
    let songs: Vec<ListItem> = get_song_list(jukebox_state);
    
    // Crea il widget List
    let songs_list = List::new(songs).block(
        Block::default()
            .title("Available Songs")
            .borders(Borders::NONE),
    )
    .highlight_style(Style::default().add_modifier(ratatui::style::Modifier::BOLD));
    
    // Crea il ListState e imposta la selezione corrente
    let mut list_state = ListState::default();
    let selected_index = jukebox_state.current_selection().position();
    list_state.select(Some(selected_index));
    
    // Renderizza con stato che gestisce automaticamente lo scrolling
    f.render_stateful_widget(songs_list, area, &mut list_state);
}
