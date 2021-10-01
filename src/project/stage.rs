use rodio::{OutputStream, OutputStreamHandle};

use super::track::Track;

use core::time::Duration;

pub struct Stage {
    stream: (OutputStream, OutputStreamHandle),
    tracks: Vec<Track>,
    transition: TransitionMode,
    length: Duration,
}
pub enum TransitionMode {
    /// As soon as a track from previous stage finishes
    /// it is replaced with a track of the same number from next stage.
    TrackAfterTrack,
    /// Waits for all the tracks to complete and starts new `Stage`
    /// from silence.
    CompleteToZero,
}

impl Default for TransitionMode {
    fn default() -> Self {
        TransitionMode::CompleteToZero
    }
}