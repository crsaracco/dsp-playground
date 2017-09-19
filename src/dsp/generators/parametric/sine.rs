//! Parametric sine wave signal generator

use dsp::evaluatable::Evaluatable;
use std::f64;

/// Sine wave generator struct.
pub struct Sine<F>
    where F: Evaluatable
{
    sample_rate: f64,
    frequency: F,
    amplitude: f64,
    phase: f64,
}

impl<F> Sine<F>
    where F: Evaluatable
{
    /// Creates a new Sine wave signal generator.
    pub fn new(sample_rate: f64, frequency: F, amplitude: f64) -> Sine<F> {
        Sine { sample_rate, frequency, amplitude, phase: 0.0}
    }
}

impl<F> Evaluatable for Sine<F>
    where F: Evaluatable
{
    fn evaluate(&mut self) -> (f32, f32) {
        let mut output = (2.0 * f64::consts::PI * (self.phase)).sin();
        // TODO: make DC offset an argument to new() for all generators so we don't have to hardcode a frequency here
        self.phase += ((self.frequency.evaluate().0 + 1000.0) as f64 / self.sample_rate).fract();

        output *= self.amplitude;
        (output as f32, output as f32)
    }
}