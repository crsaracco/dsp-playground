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
pub struct Sine<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    sample_rate: f64,  // Sample rate (for audio playback, etc) - Should be the same throughout the whole project
    amplitude: A,      // Amplitude of the Sine wave
    frequency: F,      // Frequency of the Sine wave
    offset: O,         // DC offset of the Sine wave    (+/- y axis)
    phase: f64,        // Phase offset of the Sine wave (+/- x axis, as a percent of the whole period)
}

impl<A, F, O> Sine<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    /// Creates a new Sine wave signal generator.
    pub fn new(amplitude: A, frequency: F, offset: O) -> Sine<A, F, O> {
        Sine {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl<A, F, O> Signal for Sine<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    fn evaluate(&mut self) -> (f64) {
        let amplitude = self.amplitude.evaluate();
        let frequency = self.frequency.evaluate();
        let offset = self.offset.evaluate();

        let mut output = (2.0 * f64::consts::PI * (self.phase)).sin();
        self.phase = (self.phase + frequency / self.sample_rate).fract();

        output *= amplitude;
        output += offset;

        output
    }
}