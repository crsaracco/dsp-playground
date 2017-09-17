use dsp::evaluatable::Evaluatable;

use std::f64;

pub struct SineGenerator {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    sample_number: u64,
}

impl SineGenerator {
    pub fn new(sample_rate: f64, frequency: f64, amplitude: f64) -> SineGenerator {
        SineGenerator { sample_rate, frequency, amplitude, sample_number: 0}
    }
}

impl<'a> Evaluatable for SineGenerator {
    fn evaluate(&mut self) -> (f32, f32) {
        let output = (2.0 * f64::consts::PI * (self.frequency / self.sample_rate as f64) * (self.sample_number as f64)).sin() * self.amplitude;
        self.sample_number += 1;

        (output as f32, output as f32)
    }
}