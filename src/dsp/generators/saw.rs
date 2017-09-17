use dsp::evaluatable::Evaluatable;

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

        self.output += self.frequency / self.sample_rate as f64;
        if self.output >= 1.0 {
            self.output -= 1.0;
        }

        current_output *= self.amplitude;
        (current_output as f32, current_output as f32)
    }
}