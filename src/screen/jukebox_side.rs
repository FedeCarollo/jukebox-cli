use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

use crate::jukebox_state::JukeboxState;

/// Disegna la matrice di caratteri del jukebox
pub fn render_jukebox_matrix(f: &mut Frame, area: Rect, _state: &JukeboxState) {
    use image::{GenericImageView, Pixel};
    use ratatui::text::{Line, Span};

    // Load an image and convert it to ASCII art lines

    // Try to load the image file (e.g., "jukebox.png")
    let img = image::open("jukebox.png").unwrap();

    // Resize for terminal display
    let (width, height) = (area.width as u32, area.height as u32);
    let img = img.resize_exact(width, height, image::imageops::FilterType::Nearest);

    let lines: Vec<Line> = (0..img.height())
        .map(|y| {
            let spans: Vec<Span> = (0..img.width())
                .map(|x| {
                    let pixel = img.get_pixel(x, y).to_rgb();
                    let [r, g, b] = pixel.0;
                    // Use a full block character for best density
                    Span::styled(
                        "█",
                        ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
                    )
                })
                .collect();
            Line::from(spans)
        })
        .collect();
    let paragraph =
        Paragraph::new(lines).block(Block::default().title("Jukebox").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
