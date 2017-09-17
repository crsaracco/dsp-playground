//! Square wave signal generator.
//!
//! The square wave can be considered a "basic waveform", since it has a pretty simple relationship
//! between the harmonics and the fundamental (in this case, it contains *only* odd harmonics) with
//! a falloff of -6dB/octave).
//!
//! In the time domain, the square wave oscillates between 1.0 and -1.0 only, with equal duration
//! for each. For a square-like wave with a duty cycle other than 50%, use a Pulse wave.
//!
//! In the frequency domain, the square wave is a combination of an infinite number of sine waves,
//! starting with the "fundamental frequency" (the lowest frequency, which is the `frequency`
//! field in the struct) and including every *odd* harmonic above the fundamental.
//! Each harmonic x has an amplitude of 1/x times the fundamental frequency. For example, if the
//! fundamental frequency has an amplitude of 1.00, the third harmonic would have an amplitude of
//! 0.50, the fifth harmonic would have an amplitude of 0.33, the seventh harmonic would have an
//! amplitude of 0.25, etc. Every even harmonic has an amplitude of 0.00.
//!
//! In digital synthesis, this pattern of adding of sine waves together only happens from the
//! fundamental frequency up to the Nyquist frequency (half the sampling frequency) - Otherwise,
//! you will create aliasing. NOTE that this implementation generates a square wave directly in the
//! time domain, which does not take into account the Nyquist frequency and does not band-limit
//! higher frequencies, so this implementation *might* include aliasing distortion. (???)

use dsp::evaluatable::Evaluatable;

/// Saw wave generator struct.
pub struct Square {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    sample_number: u64,
}

impl Square {
    /// Creates a new Saw wave signal generator.
    pub fn new(sample_rate: f64, frequency: f64, amplitude: f64) -> Square {
        Square { sample_rate, frequency, amplitude, sample_number: 0}
    }
}

impl Evaluatable for Square {
    fn evaluate(&mut self) -> (f32, f32) {
        let mut output = if self.sample_number < (self.sample_rate / self.frequency / 2.0) as u64 {
            -1.0
        }
        else {
            1.0
        };

        self.sample_number += 1;
        // TODO: correct?
        if self.sample_number >= (self.sample_rate / self.frequency) as u64 {
            self.sample_number = 0;
        }

        output *= self.amplitude;
        println!("{}", output);
        (output as f32, output as f32)
    }
}