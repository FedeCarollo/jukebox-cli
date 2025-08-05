use rodio::{OutputStream, OutputStreamBuilder, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::{Duration, Instant},
    vec,
};
use symphonia::core::{io::MediaSourceStream, probe::Hint};

#[derive(Debug, Clone, PartialEq)]
pub struct SongItem {
    full_path: PathBuf,
    title: String,
    position: usize,
    duration: Option<Duration>,
}

#[allow(dead_code)]
impl SongItem {
    fn new(full_path: PathBuf, title: String, position: usize) -> Self {
        Self {
            full_path: full_path.clone(),
            title,
            position,
            duration: Self::calculate_duration(full_path),
        }
    }

    fn calculate_duration(full_path: PathBuf) -> Option<Duration> {
        use std::fs::File;
        use symphonia::default::get_probe;

        let file = File::open(&full_path).ok()?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        let format = get_probe()
            .format(&hint, mss, &Default::default(), &Default::default())
            .ok()?
            .format;

        if let Some(track) = format.tracks().iter().next() {
            if let Some(time_base) = track.codec_params.time_base {
                if let Some(n_frames) = track.codec_params.n_frames {
                    let duration_secs =
                        n_frames as f64 / time_base.denom as f64 * time_base.numer as f64;
                    return Some(Duration::from_secs_f64(duration_secs));
                }
            }
        }
        None
    }

    pub fn as_path(&self) -> &Path {
        self.full_path.as_path()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    song: SongItem,
    start_time: Instant,
    elapsed_before_pause: Duration,
    is_paused: bool,
}

#[allow(dead_code)]
impl PlaybackState {
    pub fn new(song: SongItem) -> Self {
        Self {
            song,
            start_time: Instant::now(),
            elapsed_before_pause: Duration::ZERO,
            is_paused: false,
        }
    }
    
    pub fn pause(&mut self) {
        if !self.is_paused {
            self.elapsed_before_pause += self.start_time.elapsed();
            self.is_paused = true;
        }
    }
    
    pub fn resume(&mut self) {
        if self.is_paused {
            self.start_time = Instant::now();
            self.is_paused = false;
        }
    }
    
    pub fn current_position(&self) -> Duration {
        if self.is_paused {
            self.elapsed_before_pause
        } else {
            self.elapsed_before_pause + self.start_time.elapsed()
        }
    }
    
    pub fn song(&self) -> &SongItem {
        &self.song
    }
    
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}

#[allow(unused)]
pub struct JukeboxState {
    initial_path: PathBuf,
    current_selection: SongItem,
    playlist: Vec<SongItem>,
    current_playback: Option<PlaybackState>,
    volume: u8,
    stream_handle: OutputStream,
    sink: Option<Sink>,
}

#[allow(dead_code)]
impl JukeboxState {
    pub fn new(initial_path: &Path) -> Self {
        // Read directory and get all mp3 files
        let mut playlist = vec![];
        if let Ok(entries) = std::fs::read_dir(initial_path) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "mp3" {
                        let title = entry.file_name().to_string_lossy().to_string();
                        playlist.push(SongItem::new(entry.path(), title, playlist.len()));
                    }
                }
            }
        }

        let initial_selection = playlist.first().cloned().unwrap_or_else(|| {
            SongItem::new(PathBuf::from("."), "No songs available".to_string(), 0)
        });

        let stream_handle =
            OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");

        Self {
            initial_path: initial_path.to_path_buf(),
            current_selection: initial_selection,
            playlist,
            current_playback: None,
            volume: 50, // Default volume
            stream_handle,
            sink: None,
        }
    }

    pub fn current_selection(&self) -> &SongItem {
        &self.current_selection
    }

    pub fn play(&mut self) {
        // Check if we have a current playback and if it's the same song as selected
        if let Some(playback) = &mut self.current_playback {
            let current_song = &self.current_selection;
            let playing_song = playback.song();
            
            // If it's the same song and paused, resume
            if current_song == playing_song && playback.is_paused() {
                if let Some(sink) = &self.sink {
                    sink.play();
                    playback.resume();
                }
                return;
            }
            
            // If it's a different song, we'll start the new one (fall through to start new song)
        }
        
        // Start new song (either no current playback or different song selected)
        if let Some(song) = self.playlist.get(self.current_selection.position) {
            // Clone the needed data before calling self.stop()
            let song_full_path = song.full_path.clone();
            let song_clone = song.clone();

            // Stop current playback if any
            self.stop();

            let file =
                BufReader::new(File::open(&song_full_path).expect("Failed to open song file"));
            let sink = rodio::play(&self.stream_handle.mixer(), file).expect("Failed to play song");
            sink.set_volume(self.volume as f32 / 100.0);

            self.sink = Some(sink);
            self.current_playback = Some(PlaybackState::new(song_clone));
        }
    }
    
    pub fn pause(&mut self) {
        if let Some(playback) = &mut self.current_playback {
            if !playback.is_paused() {
                if let Some(sink) = &self.sink {
                    sink.pause();
                    playback.pause();
                }
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
            sink.sleep_until_end();
        }
        self.sink = None;
        self.current_playback = None;
    }

    pub fn add_volume(&mut self, amount: u8) {
        self.volume = (self.volume.saturating_add(amount)).min(100);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume as f32 / 100.0);
        }
    }

    pub fn sub_volume(&mut self, amount: u8) {
        self.volume = self.volume.saturating_sub(amount);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume as f32 / 100.0);
        }
    }

    pub fn move_selection(&mut self, direction: i32) {
        if self.playlist.is_empty() {
            return;
        }

        let current_pos = self.current_selection.position as i32;
        let new_pos = (current_pos + direction)
            .max(0)
            .min(self.playlist.len() as i32 - 1) as usize;

        if let Some(song) = self.playlist.get(new_pos) {
            self.current_selection = song.clone();
        }
    }

    pub fn playlist(&self) -> &[SongItem] {
        &self.playlist
    }

    pub fn currently_playing(&self) -> Option<&SongItem> {
        self.current_playback.as_ref().map(|p| p.song())
    }

    pub fn is_playing(&self) -> bool {
        self.sink.is_some() && 
        self.current_playback.as_ref().map_or(false, |p| !p.is_paused())
    }
    
    pub fn is_song_finished(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.empty()
        } else {
            false
        }
    }
    
    pub fn handle_song_end(&mut self) {
        if self.is_song_finished() {
            // Se non siamo all'ultima canzone, passa alla successiva
            if self.current_selection.position < self.playlist.len() - 1 {
                self.move_selection(1);
                self.play();
            } else {
                // Se siamo all'ultima canzone, ferma la riproduzione
                self.stop();
            }
        }
    }
    
    pub fn current_playback_position(&self) -> Duration {
        self.current_playback.as_ref()
            .map_or(Duration::ZERO, |p| p.current_position())
    }
    
    pub fn progress_ratio(&self) -> f32 {
        if let Some(playback) = &self.current_playback {
            if let Some(duration) = playback.song().duration {
                let pos = playback.current_position();
                if duration.as_secs() > 0 {
                    return (pos.as_secs_f32() / duration.as_secs_f32()).min(1.0);
                }
            }
        }
        0.0
    }
    
    pub fn volume(&self) -> u8 {
        self.volume
    }
}
