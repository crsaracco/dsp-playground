//! DSP traits

/// Signal trait
///
/// All signals must implement this trait. This allows us to do some pretty powerful stuff, like:
///  - Add signals together to create a new signal
///  - Negate a signal to create a new signal
///  - Make generators that generate signals, such as Sine waves, Square waves, Saw waves, etc.
///  - Allow generators to accept OTHER signals as inputs to control the frequency, amplitude, etc.
///
/// Note that this type of Signal is single-channel only.
/// TODO: create a trait for double-channel (stereo), and have some sort of conversion between them

pub trait Signal {
    /// When requested, all signals must produce a f64 sample.
    /// Note that PortAudio does not accept f64s; it will downsample to f32 for output.
    fn evaluate(&mut self) -> f64;
}