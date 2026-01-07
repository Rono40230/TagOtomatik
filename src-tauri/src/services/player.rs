use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// EQ Structs migrated to separate file equalizer.rs to reduce file size
use super::equalizer::{EqParams, EqualizerSource};


pub enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    Seek(f32),
    SetEq(f32, f32, f32), // Bass, Mid, Treble
}

pub struct AudioPlayerState {
    tx: Mutex<Sender<PlayerCommand>>,
    #[allow(dead_code)]
    eq_params: Arc<Mutex<EqParams>>,
}

impl Default for AudioPlayerState {
    fn default() -> Self {
        Self::new()
    }
}


impl AudioPlayerState {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let eq_params = Arc::new(Mutex::new(EqParams::default()));
        let eq_params_thread = eq_params.clone();

        thread::spawn(move || {
            // Initialize audio output in this thread
            let stream_result = OutputStream::try_default();
            if let Err(e) = &stream_result {
                eprintln!("Failed to get output stream: {}", e);
                return;
            }
            let (_stream, stream_handle) = stream_result.unwrap();

            // Keep track of the current sink
            let mut sink = Sink::try_new(&stream_handle).ok();

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    PlayerCommand::Play(path) => {
                        // Stop previous track
                        if let Some(s) = &sink {
                            s.stop();
                        }

                        // Create new sink for new track
                        sink = Sink::try_new(&stream_handle).ok();

                        if let Some(s) = &sink {
                            match File::open(&path) {
                                Ok(file) => {
                                    let reader = BufReader::new(file);
                                    match Decoder::new(reader) {
                                        Ok(source) => {
                                            // WRAP SOURCE WITH EQ
                                            let convert_source = source.convert_samples::<f32>();
                                            let eq_source = EqualizerSource::new(convert_source, eq_params_thread.clone());
                                            
                                            s.append(eq_source);
                                            s.play();
                                        }
                                        Err(e) => eprintln!("Error decoding file {}: {}", path, e),
                                    }
                                }
                                Err(e) => eprintln!("Error opening file {}: {}", path, e),
                            }
                        }
                    }
                    PlayerCommand::Pause => {
                        if let Some(s) = &sink {
                            s.pause();
                        }
                    }
                    PlayerCommand::Resume => {
                        if let Some(s) = &sink {
                            s.play();
                        }
                    }
                    PlayerCommand::Stop => {
                        if let Some(s) = &sink {
                            s.stop();
                        }
                        sink = None; // Clear sink
                    }
                    PlayerCommand::SetVolume(vol) => {
                        if let Some(s) = &sink {
                            s.set_volume(vol);
                        }
                    }
                    PlayerCommand::Seek(seconds) => {
                        if let Some(s) = &sink {
                            if let Err(e) = s.try_seek(Duration::from_secs_f32(seconds)) {
                                eprintln!("Error seeking to {}s: {}", seconds, e);
                            }
                        }
                    }
                    PlayerCommand::SetEq(b, m, t) => {
                        if let Ok(mut p) = eq_params_thread.lock() {
                            p.bass_gain = b;
                            p.mid_gain = m;
                            p.treble_gain = t;
                        }
                    }
                }
            }
        });

        Self { 
            tx: Mutex::new(tx),
            eq_params 
        }
    }

    pub fn play(&self, path: &str) -> Result<(), String> {
        self.tx
            .lock()
            .map_err(|_| "Failed to lock player state".to_string())?
            .send(PlayerCommand::Play(path.to_string()))
            .map_err(|e| e.to_string())
    }

    pub fn pause(&self) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::Pause);
        }
    }

    pub fn resume(&self) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::Resume);
        }
    }

    pub fn stop(&self) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::Stop);
        }
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::SetVolume(volume));
        }
    }

    pub fn seek(&self, time: f32) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::Seek(time));
        }
    }

    pub fn set_eq(&self, bass: f32, mid: f32, treble: f32) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(PlayerCommand::SetEq(bass, mid, treble));
        }
    }
}
