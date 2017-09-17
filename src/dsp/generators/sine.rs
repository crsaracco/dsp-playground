//! Sine wave signal generator

use dsp::evaluatable::Evaluatable;
use std::f64;

/// Sine wave generator struct.
pub struct Sine {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    sample_number: u64,
}

impl Sine {
    /// Creates a new Sine wave signal generator.
    pub fn new(sample_rate: f64, frequency: f64, amplitude: f64) -> Sine {
        Sine { sample_rate, frequency, amplitude, sample_number: 0}
    }
}

impl Evaluatable for Sine {
    fn evaluate(&mut self) -> (f32, f32) {
        let mut output = (2.0 * f64::consts::PI * (self.frequency / self.sample_rate as f64) * (self.sample_number as f64)).sin();
        self.sample_number += 1;

        output *= self.amplitude;
        (output as f32, output as f32)
    }
}