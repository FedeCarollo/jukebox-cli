use ratatui::text::Span;
use std::time::{Duration, Instant};
use std::fs;
use image::{GenericImageView, Pixel};
use rand::Rng;

#[derive(Clone)]
pub struct FloatingNote {
    x: u16,
    y: u16,
    pixels: Vec<Vec<Span<'static>>>,
    spawn_time: Instant,
}

impl FloatingNote {
    fn new(x: u16, y: u16, note_images: &[image::DynamicImage]) -> Option<Self> {
        if note_images.is_empty() {
            return None;
        }
        
        let mut rng = rand::rng();
        let note_index = rng.random_range(0..note_images.len());
        let note_image = &note_images[note_index];
        
        // Random size tra 6x6 e 12x12
        let size = rng.random_range(6..=12);
        let resized = note_image.resize_exact(
            size as u32,
            size as u32,
            image::imageops::FilterType::Nearest,
        );
        
        // Converti in pixels con trasparenza
        let mut pixels = Vec::new();
        for py in 0..size {
            let mut row = Vec::new();
            for px in 0..size {
                let pixel = resized.get_pixel(px as u32, py as u32);
                let [r, g, b, a] = pixel.0;
                
                if a > 128 { // Pixel visibile
                    row.push(Span::styled(
                        "█",
                        ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
                    ));
                } else { // Pixel trasparente
                    row.push(Span::raw(" "));
                }
            }
            pixels.push(row);
        }
        
        Some(Self {
            x,
            y,
            pixels,
            spawn_time: Instant::now(),
        })
    }
    
    fn is_expired(&self) -> bool {
        self.spawn_time.elapsed() > Duration::from_secs(1)
    }

}

pub struct CanvasState {
    cached_background: Option<Vec<Vec<Span<'static>>>>,
    floating_notes: Vec<FloatingNote>,
    note_images: Vec<image::DynamicImage>,
    last_size: (u16, u16),
}

impl CanvasState {
    pub fn new() -> Self {
        // Carica le immagini delle note
        let note_images = fs::read_dir("img/notes")
            .unwrap_or_else(|_| panic!("Cartella img/notes deve esistere"))
            .filter_map(Result::ok)
            .filter_map(|entry| image::open(entry.path()).ok())
            .collect::<Vec<_>>();
        
        if note_images.is_empty() {
            panic!("Nessuna immagine trovata in img/notes");
        }
        
        Self {
            cached_background: None,
            floating_notes: Vec::new(),
            note_images,
            last_size: (0, 0),
        }
    }
    
    pub fn update_is_playing(&mut self, is_playing: bool) {
        // Se non sta suonando, rimuovi tutte le note
        if !is_playing {
            self.floating_notes.clear();
        }
    }
    
    pub fn update_notes(&mut self, width: u16, height: u16, is_playing: bool) {
        if !is_playing {
            return;
        }
        
        let mut rng = rand::rng();
        
        self.floating_notes.retain(|note| !note.is_expired());
        
        // Add a note with 50% probability
        if rng.random_bool(0.5) && self.floating_notes.len() < 6 && width > 20 && height > 20 {
            let max_note_size = 12; // Dimensione massima nota
            let actual_width = width.saturating_sub(2);
            let actual_height = height.saturating_sub(2);
            
            if actual_width > max_note_size && actual_height > max_note_size {
                let x = rng.random_range(2..actual_width.saturating_sub(max_note_size));
                let y = rng.random_range(2..actual_height.saturating_sub(max_note_size));
                
                if let Some(note) = FloatingNote::new(x, y, &self.note_images) {
                    self.floating_notes.push(note);
                }
            }
        }
    }
    
    pub fn get_canvas(&mut self, width: u16, height: u16) -> Vec<Vec<Span<'static>>> {
        let actual_width = width.saturating_sub(2);
        let actual_height = height.saturating_sub(2);
        
        if actual_width == 0 || actual_height == 0 {
            return vec![vec![Span::raw(" "); 1]; 1];
        }
        
        // Controlla se dobbiamo ricreare il background
        let need_new_background = self.last_size != (actual_width, actual_height);
        
        let mut canvas = if need_new_background {
            let new_background = self.create_background(actual_width, actual_height);
            self.cached_background = Some(new_background.clone());
            self.last_size = (actual_width, actual_height);
            new_background
        } else {
            // Usa background cached
            if let Some(ref background) = self.cached_background {
                background.clone()
            } else {
                self.create_background(actual_width, actual_height)
            }
        };
        
        // Add notes to the canvas
        for note in &self.floating_notes {
            // Disegna la nota sul canvas
            for (row_idx, row) in note.pixels.iter().enumerate() {
                let canvas_y = note.y + row_idx as u16;
                if canvas_y >= actual_height {
                    break;
                }
                
                for (col_idx, pixel) in row.iter().enumerate() {
                    let canvas_x = note.x + col_idx as u16;
                    if canvas_x >= actual_width {
                        break;
                    }
                    
                    // Only if pixel hold content
                    if pixel.content != " " {
                        canvas[canvas_y as usize][canvas_x as usize] = pixel.clone();
                    }
                }
            }
        }
        
        canvas
    }
    
    fn create_background(&self, width: u16, height: u16) -> Vec<Vec<Span<'static>>> {
        match image::open("img/jukebox.png") {
            Ok(img) => {
                let resized = img.resize_exact(
                    width as u32,
                    height as u32,
                    image::imageops::FilterType::Nearest,
                );
                
                (0..height)
                    .map(|y| {
                        (0..width)
                            .map(|x| {
                                let pixel = resized.get_pixel(x as u32, y as u32).to_rgb();
                                let [r, g, b] = pixel.0;
                                Span::styled(
                                    "█",
                                    ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
                                )
                            })
                            .collect()
                    })
                    .collect()
            }
            Err(_) => {
                // Simple fallback
                vec![vec![Span::styled("█", ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray)); width as usize]; height as usize]
            }
        }
    }
}
