use core::time::Duration;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

use rodio::{OutputStream, Source};

/// An alias of a specific type implementing `rodio::Source` trait
/// To create an instance of the type from a file do:
/// ```
/// let file = File::open("assets/example.flac");
/// let buf = BufReader::new(&file);
/// let source = Box::new(
///                 rodio::Decoder::new(buf)
///                 .unwrap()
///                 .convert_samples()
/// )
/// ```
type SoundSource = Box<dyn rodio::Source<Item = f32> + Send>;

pub struct Sound {
    volume: f32,
    length: Duration,
    path: String,
    source: SoundSource,
}

impl Sound {
    pub fn new(path: String) -> Result<Sound, Box<dyn Error>> {
        let file = File::open(&path)?;
        let buf = BufReader::new(file);
        let source = Box::new(rodio::Decoder::new(buf)?.convert_samples());
        let length = source.total_duration().unwrap();
        Ok(Sound {
            volume: 1.0,
            length,
            path,
            source,
        })
    }
    pub fn length(&self) -> Duration {
        self.length
    }
    pub fn set_volume(mut self, vol: f32) -> Self {
        self.source = Box::new(self.source.amplify(vol));
        self.volume = &self.volume * vol;
        self
    }
    pub fn preview_duration(self, dur: Duration) -> Result<Sound, Box<dyn Error>> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;
        let clone = Sound::new(self.path)?.set_volume(self.volume);
        sink.append(self.source);
        std::thread::sleep(dur);
        Ok(clone)
    }
    pub fn preview(self) -> Result<Sound, Box<dyn Error>> {
        let dur = self.length;
        Ok(self.preview_duration(dur)?)
    }
    pub fn clone_source(&self) -> SoundSource {
        let buf = BufReader::new(File::open(&self.path).unwrap());
        Box::new(rodio::Decoder::new(buf).unwrap().convert_samples())
    }
}