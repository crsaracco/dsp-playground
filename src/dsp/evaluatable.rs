//! Evaluatable trait that all Signals must implement for use in PortAudio

/// Evaluatable trait
pub trait Evaluatable {
    /// when called, the signal produces the next (two-channel) sample to be played.
    fn evaluate(&mut self) -> (f32, f32);
}