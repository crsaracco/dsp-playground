//! Saw wave signal generator.
//!
//! The saw wave can be considered a "basic waveform", since it has a pretty simple relationship
//! between the harmonics and the fundamental (in this case, it contains all harmonics (odd *and*
//! even) with a falloff of -6dB/octave).
//!
//! In the time domain, the conventional saw wave has the zeroth sample starting at -1.00, and
//! consecutive samples ramp upward until it hits 1.00, then sharply drops to -1.00 and starts over.
//! The rate at which the samples ramp up depend on the frequency of the saw wave. To get an
//! "inverse saw wave" (one that starts at 1.00 and ramps downward until -1.00), you can simply make
//! a conventional saw wave with the desired frequency and amplitude and negate the signal.
//!
//! In the frequency domain, the saw wave is a combination of an infinite number of sine waves,
//! starting with the "fundamental frequency" (the lowest frequency, which is the `frequency`
//! field in the struct) and including every (even and odd) harmonic above the fundamental.
//! Each harmonic x has an amplitude of 1/x times the fundamental frequency. For example, if the
//! fundamental frequency has an amplitude of 1.00, the second harmonic would have an amplitude of
//! 0.50, the third harmonic would have an amplitude of 0.33, the fourth harmonic would have an
//! amplitude of 0.25, etc.
//!
//! In digital synthesis, this pattern of adding of sine waves together only happens from the
//! fundamental frequency up to the Nyquist frequency (half the sampling frequency) - Otherwise,
//! you will create aliasing. NOTE that this implementation generates a saw wave directly in the
//! time domain, which does not take into account the Nyquist frequency and does not band-limit
//! higher frequencies, so this implementation *might* include aliasing distortion. (???)

use dsp::evaluatable::Evaluatable;

/// Saw wave generator struct.
pub struct Saw {
    sample_rate: f64,
    frequency: f64,
    amplitude: f64,
    output: f64,
}

impl Saw {
    /// Creates a new Saw wave signal generator.
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