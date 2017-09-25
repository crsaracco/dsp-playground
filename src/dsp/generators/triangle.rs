//! Triangle wave generator

use dsp::traits::Signal;

/// Triangle wave generator struct.
pub struct Triangle<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    sample_rate: f64,  // Sample rate (for audio playback, etc) - Should be the same throughout the whole project
    amplitude: A,      // Amplitude of the Square wave
    frequency: F,      // Frequency of the Square wave
    offset: O,         // DC offset of the Square wave    (+/- y axis)
    phase: f64,        // Phase offset of the Square wave (+/- x axis, as a percent of the whole period)
}

impl<A, F, O> Triangle<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    /// Creates a new Saw wave signal generator.
    pub fn new(amplitude: A, frequency: F, offset: O) -> Triangle<A, F, O> {
        Triangle {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl<A, F, O> Signal for Triangle<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    fn evaluate(&mut self) -> (f64) {
        let amplitude = self.amplitude.evaluate();
        let frequency = self.frequency.evaluate();
        let offset = self.offset.evaluate();

        let mut output = match self.phase {
            n if n <= 0.5 => (self.phase*2.0)*2.0 - 1.0,
            _ => ((1.0 - self.phase)*2.0)*2.0 - 1.0,
        };
        let last_phase = self.phase;
        self.phase = (self.phase + frequency / self.sample_rate).fract();

        output *= amplitude;
        output += offset;

        output
    }
}