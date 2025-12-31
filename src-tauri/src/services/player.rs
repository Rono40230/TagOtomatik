use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;

pub enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub struct AudioPlayerState {
    tx: Mutex<Sender<PlayerCommand>>,
}

impl Default for AudioPlayerState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayerState {
    pub fn new() -> Self {
        let (tx, rx) = channel();

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
                                            s.append(source);
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
                }
            }
        });

        Self { tx: Mutex::new(tx) }
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
}
