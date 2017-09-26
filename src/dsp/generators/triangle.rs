//! Triangle wave generator

use dsp::traits::Signal;

/// Triangle wave generator struct.
pub struct Triangle {
    sample_rate: f64,  // Sample rate (for audio playback, etc) - Should be the same throughout the whole project
    amplitude: f64,    // Amplitude of the Square wave
    frequency: f64,    // Frequency of the Square wave
    offset: f64,       // DC offset of the Square wave    (+/- y axis)
    phase: f64,        // Phase offset of the Square wave (+/- x axis, as a percent of the whole period)
}

impl Triangle {
    /// Creates a new Saw wave signal generator.
    pub fn new(amplitude: f64, frequency: f64, offset: f64) -> Triangle {
        Triangle {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl Signal for Triangle {
    fn evaluate(&mut self) -> (f64) {
        let mut output = match self.phase {
            n if n <= 0.5 => (self.phase*2.0)*2.0 - 1.0,
            _ => ((1.0 - self.phase)*2.0)*2.0 - 1.0,
        };
        self.phase = (self.phase + self.frequency / self.sample_rate).fract();

        // Transform the signal, taking into account the amplitude and DC offset
        output *= self.amplitude;
        output += self.offset;

        // Return the output
        output
    }
}