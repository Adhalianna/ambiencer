use rodio::{OutputStream, OutputStreamHandle};
use super::track::Track;

use core::time::Duration;

pub struct Stage {
    stream: (OutputStream, OutputStreamHandle),
    tracks: Vec<Track>,
    in_fader: Box<dyn FadeIn>,
    filler: Box<dyn Fill>,
    out_fader: Box<dyn FadeOut>,
    length: Duration,
}

pub trait FadeIn {
    fn fade_in(track: &Track, stream: &OutputStreamHandle)
    where Self: Sized;
}

pub trait Fill {
    fn fill(track: &Track, stream: &OutputStreamHandle)
    where Self: Sized;
}

pub trait FadeOut {
    fn fade_out(track: &Track, stream: &OutputStreamHandle)
    where Self: Sized;
}

#[cfg(test)]
mod test{
    use std::time::Duration;
    use rodio::OutputStreamHandle;
    use crate::project::Track;
    use super::{Stage, Fill, FadeIn, FadeOut};

    // Very dummy trait implementations for a 'it compiles' test
    struct DummyFiller {}
    impl Fill for DummyFiller {
        fn fill(track: &Track, stream: &OutputStreamHandle)
        where Self: Sized {
            ()
        }
    }
    struct DummyFadeOut {}
    impl FadeOut for DummyFadeOut {
        fn fade_out(track: &Track, stream: &OutputStreamHandle)
        where Self: Sized {
            ()
        }
    }
    struct DummyFadeIn {}
    impl FadeIn for DummyFadeIn {
        fn fade_in(track: &Track, stream: &OutputStreamHandle)
        where Self: Sized {
            ()
        }
    }

    #[test]
    #[ignore = "No further code would ever compile if this didn't pass."]
    fn it_compiles(){
        let _ = Stage {
            stream: rodio::OutputStream::try_default().unwrap(),
            tracks: Vec::new(),
            in_fader: Box::new(DummyFadeIn {}),
            filler: Box::new(DummyFiller {}),
            out_fader: Box::new(DummyFadeOut {}),
            length: Duration::from_secs(60),
        };
    }
}