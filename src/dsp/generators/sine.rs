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

use dsp::traits::Signal;
use std::f64;

/// Sine wave generator struct.
pub struct Sine {
    sample_rate: f64,  // Sample rate (for audio playback, etc) - Should be the same throughout the whole project
    amplitude: f64,    // Amplitude of the Sine wave
    frequency: f64,    // Frequency of the Sine wave
    offset: f64,       // DC offset of the Sine wave    (+/- y axis)
    phase: f64,        // Phase offset of the Sine wave (+/- x axis, as a percent of the whole period)
}

impl Sine {
    /// Creates a new Sine wave signal generator using user-input f64s.
    pub fn new(amplitude: f64, frequency: f64, offset: f64) -> Sine {
        Sine {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl Signal for Sine {
    fn evaluate(&mut self) -> f64 {
        let mut output = (2.0 * f64::consts::PI * (self.phase)).sin();
        self.phase = (self.phase + self.frequency / self.sample_rate).fract();

        // Transform the signal, taking into account the amplitude and DC offset
        output *= self.amplitude;
        output += self.offset;

        // Return the output
        output
    }
}