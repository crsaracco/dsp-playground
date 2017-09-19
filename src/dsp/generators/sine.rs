//! Sine wave signal generator
//!
//! The sine wave can be considered a "basic waveform", since it has a pretty simple relationship
//! between the harmonics and the fundamental (in this case, there are no harmonics; only the
//! fundamental).
//!
//! The sine wave is the basic building block of sound: any sound can be constructed by adding
//! multiple sine waves together with varying frequencies and amplitudes.
//!
//! In the frequency domain, a sine wave represents a "pure tone". It consists of only one
//! frequency: the frequency of the sine wave itself.

use dsp::traits::Evaluatable;
use std::f64;

/// Sine wave generator struct.
pub struct Sine {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    phase: f64,
}

impl Sine {
    /// Creates a new Sine wave signal generator.
    pub fn new(sample_rate: f64, frequency: f64, amplitude: f64) -> Sine {
        Sine { sample_rate, frequency, amplitude, phase: 0.0}
    }
}

impl Evaluatable for Sine {
    fn evaluate(&mut self) -> (f32, f32) {
        let mut output = (2.0 * f64::consts::PI * (self.phase)).sin();
        self.phase += (self.frequency / self.sample_rate).fract();

        output *= self.amplitude;
        (output as f32, output as f32)
    }
}