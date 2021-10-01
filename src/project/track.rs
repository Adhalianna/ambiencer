use rodio::OutputStreamHandle;
use rodio::Sink as SimpleSink;
use rodio::SpatialSink;
use rodio::Source;

use std::fmt::Debug;
use std::error::Error;
use core::time::Duration;

use crate::project::sound::Sound;


static EAR_SPACING: f32 = 5.0;

pub struct Track {
    stream: OutputStreamHandle,
    sink: Sink,
    /// `volume` of the `Track` is the base volume to be used
    /// rather than the volume of a `rodio::Sink` which may vary
    /// when a `Stage` transition occures.
    volume: f32,
    position: f32,
    elements: Vec<Sound>,
}

/// Ambiencer's `Sink` enum is used as an additional layer of
/// abstraction over `rodio::Sink` and `rodio::SpatialSink`
/// which oddly do not share a trait.
enum Sink {
    Simple(SimpleSink),
    Spatial(SpatialSink),
}

//NOTE: this is ugly, shouldn't there be a neatier way?
impl Sink {
    pub fn play(&self) {
        match self {
            Sink::Simple(s) => s.sleep_until_end(),
            Sink::Spatial(s) => s.sleep_until_end(),
        }
    }
    pub fn append(
        &self,
        el: impl Source<Item = impl rodio::Sample + Send + Debug> + Send + 'static,
    ) {
        match self {
            Sink::Simple(s) => s.append(el),
            Sink::Spatial(s) => s.append(el),
        }
    }
}

impl Track {
    //TODO: Make it possible to specify a device
    pub fn new(streamhandle: OutputStreamHandle) -> Result<Track, Box<dyn Error>> {
        let sink = Sink::Simple(SimpleSink::try_new(&streamhandle)?);
        Ok(Track {
            stream: streamhandle,
            sink,
            volume: 1.0,
            position: 0.0,
            elements: Vec::new(),
        })
    }
    pub fn add(&mut self, elem: Sound) {
        self.elements.push(elem);
    }
    //NOTE: The API for those `add` functions seems somewhat inconsistent with `Sound`
    pub fn add_from_path(&mut self, filepath: String) -> Result<(), Box<dyn Error>> {
        self.elements.push(Sound::new(filepath)?);
        Ok(())
    }
    pub fn length(&self) -> Duration {
        self.elements
            .iter()
            .fold(Duration::new(0, 0), |sum, el| sum + el.length())
    }
    pub fn count(&self) -> usize {
        self.elements.len()
    }
    pub fn set_volume(&mut self, vol: f32) -> Result<f32, ()> {
        self.volume *= vol;
        match &self.sink {
            Sink::Simple(s) => s.set_volume(vol),
            Sink::Spatial(s) => s.set_volume(vol),
        }
        Ok(self.volume)
    }
    pub fn set_position(&mut self, pos: f32) -> Result<f32, Box<dyn Error>> {
        match &self.sink {
            Sink::Simple(_s) => {
                if pos == 0.0 {
                    return Ok(pos);
                } else {
                    self.sink = Sink::Spatial(SpatialSink::try_new(
                        &self.stream,
                        [pos, 0.0, 0.0],
                        [-EAR_SPACING, 0.0, 0.0],
                        [EAR_SPACING, 0.0, 0.0],
                    )?);
                    self.position = pos;
                }
            }
            Sink::Spatial(_s) => {
                self.sink = Sink::Spatial(SpatialSink::try_new(
                    &self.stream,
                    [pos, 0.0, 0.0],
                    [-EAR_SPACING, 0.0, 0.0],
                    [EAR_SPACING, 0.0, 0.0],
                )?);
                self.position = pos;
            }
        }
        Ok(self.position)
    }
    pub fn reset_position(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.sink {
            Sink::Simple(_s) => return Ok(()),
            Sink::Spatial(_s) => {
                self.sink = Sink::Simple(SimpleSink::try_new(&self.stream)?);
                return Ok(());
            }
        }
    }
    fn fill_to_duration_cutting(&self, mut dur: Duration) {
        let mut elements_loop = self.elements.iter().cycle();
        //let mut count = 0;
        loop {
            let el = elements_loop.next().unwrap();
            if dur <= el.length() {
                self.sink.append(el.clone_source().take_duration(dur));
                return;
                //return count + 1;
            } else {
                self.sink.append(el.clone_source());
            }
            dur -= el.length();
            //count += 1;
        }
    }
    /// Fill the track with sounds playing no longer than specified duration
    /// and return the first sound that could not fit in.
    fn fill_no_longer_than(&self, mut dur: Duration) -> &Sound {
        let mut elements_loop = self.elements.iter().cycle();
        loop {
            let el = elements_loop.next().unwrap();
            if dur <= el.length() {
                return el;
            }
            else {
                self.sink.append(el.clone_source());
            }
            dur -= el.length();
        }
    }
    pub fn preview_duration(&self, dur: Duration) {
        self.fill_to_duration_cutting(dur);
        std::thread::sleep(dur);
    }
}