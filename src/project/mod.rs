extern crate rodio;

mod stage;
mod track;
mod sound;

pub use stage::*;
pub use track::*;
pub use sound::*;


struct Project {
    name: String,
    filepath: String,
    stages: Vec<Stage>
}


#[cfg(test)]
mod test {
    use super::sound::Sound;
    use super::track::Track;
    use super::stage::Stage;
    use std::time::Duration;

    // Most of the tests here are marked with `#[ignore]` so that they
    // do not run by default. They may be bad for automated testing.

    #[test]
    #[ignore = "Meant to be tested by listening"]
    fn play_sound() {
        let sound = Sound::new("assets/birds_loop_1.flac".to_string()).unwrap();
        sound.preview_duration(Duration::from_secs(5)).unwrap();
    }
    #[test]
    #[ignore = "Meant to be tested by listening"]
    fn play_sound_louder() {
        let sound = Sound::new("assets/birds_loop_1.flac".to_string()).unwrap();
        let sound = sound
            .set_volume(1.5)
            .preview_duration(Duration::from_secs(5))
            .unwrap();
        sound
            .set_volume(2.5)
            .preview_duration(Duration::from_secs(5))
            .unwrap();
    }
    #[test]
    fn track_length() {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut track = Track::new(handle).unwrap();
        track.add_from_path("assets/birds_loop_1.flac".to_string()).unwrap();
        track.add_from_path("assets/birds_loop_1.flac".to_string()).unwrap();
        let len = track.length();
        dbg!(len);
        assert_eq!(len, Duration::new(40, 0));
    }
    #[test]
    #[ignore = "Meant to be tested by listening"]
    fn play_track() {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut track = Track::new(handle).unwrap();
        track.add_from_path("assets/birds_loop_1.flac".to_string()).unwrap();
        track.add_from_path("assets/birds_loop_2.flac".to_string()).unwrap();
        track.set_volume(2.0).unwrap();
        track.preview_duration(Duration::new(5, 0));
    }
    #[test]
    #[ignore = "Meant to be tested by listening"]
    fn play_quiet_sound_loud_track() {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut track = Track::new(handle).unwrap();
        track.add(Sound::new("assets/birds_loop_1.flac".to_string())
            .unwrap()
            .set_volume(0.1)
        );
        track.set_volume(3.0).unwrap();
        track.preview_duration(Duration::new(3, 0));
    }
}
