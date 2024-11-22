// audio.rs

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
}

impl AudioPlayer {
    pub fn new(music_file: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open(music_file).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.set_volume(0.5);

        AudioPlayer {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
        }
    }

    pub fn play(&self) {
        self.sink.lock().unwrap().play();
    }

    pub fn stop(&self) {
        self.sink.lock().unwrap().stop();
    }

    pub fn play_loop(&self) {
        let mut sink_guard = self.sink.lock().unwrap();
        if sink_guard.empty() {
            let file = BufReader::new(File::open("assets/death_star_alarm.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            sink_guard.append(source.repeat_infinite()); // Reproducir en bucle
        }
        sink_guard.play();
    }
}