use dsp::evaluatable::Evaluatable;
use std::f64;

// Sine wave generator
pub struct Sine {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    sample_number: u64,
}

impl Sine {
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

// Saw wave generator
pub struct Saw {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    output: f64,
}

impl Saw {
    pub fn new(sample_rate: f64, frequency: f64, amplitude: f64) -> Saw {
        Saw { sample_rate, frequency, amplitude, output: 0.0}
    }
}

impl Evaluatable for Saw {
    fn evaluate(&mut self) -> (f32, f32) {
        let mut current_output = self.output;

        self.output += (self.frequency / self.sample_rate as f64);
        if self.output >= 1.0 {
            self.output -= 1.0;
        }

        current_output *= self.amplitude;
        (current_output as f32, current_output as f32)
    }
}