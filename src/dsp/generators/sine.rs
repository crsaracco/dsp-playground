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
    amplitude: Box<Signal>,      // Amplitude of the Sine wave
    frequency: Box<Signal>,      // Frequency of the Sine wave
    offset: Box<Signal>,         // DC offset of the Sine wave    (+/- y axis)
    phase: f64,        // Phase offset of the Sine wave (+/- x axis, as a percent of the whole period)
}

impl Sine {
    /// Creates a new Sine wave signal generator using user-input f64s.
    pub fn new(amplitude: f64, frequency: f64, offset: f64) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: 44100.0,
            amplitude: Box::new(amplitude),
            frequency: Box::new(frequency),
            offset: Box::new(offset),
            phase: 0.0,
        })
    }

    /// Creates a new Sine wave signal generator using default values.
    /// Users will probably want to set values using the `set_foo()` functions.
    pub fn default() -> Box<Sine> {
        Box::new(Sine {
            sample_rate: 44100.0,
            amplitude: Box::new(1.0),
            frequency: Box::new(440.0),
            offset: Box::new(0.0),
            phase: 0.0,
        })
    }

    /// Set the amplitude of a Sine wave and return a new sine
    pub fn set_amplitude(self, amplitude: Box<Signal>) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude,
            frequency: self.frequency,
            offset: self.offset,
            phase: self.phase,
        })
    }

    /// Set the amplitude of a Sine wave and return a new sine
    pub fn set_amplitude_f64(self, amplitude: f64) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude: Box::new(amplitude),
            frequency: self.frequency,
            offset: self.offset,
            phase: self.phase,
        })
    }

    /// Set the frequency of a Sine wave and return a new sine
    pub fn set_frequency(self, frequency: Box<Signal>) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude: self.amplitude,
            frequency,
            offset: self.offset,
            phase: self.phase,
        })
    }

    /// Set the frequency of a Sine wave and return a new sine
    pub fn set_frequency_f64(self, frequency: f64) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude: self.amplitude,
            frequency: Box::new(frequency),
            offset: self.offset,
            phase: self.phase,
        })
    }

    /// Set the offset of a Sine wave and return a new sine
    pub fn set_offset(self, offset: Box<Signal>) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude: self.amplitude,
            frequency: self.frequency,
            offset,
            phase: self.phase,
        })
    }

    /// Set the offset of a Sine wave and return a new sine
    pub fn set_offset_f64(self, offset: f64) -> Box<Sine> {
        Box::new(Sine {
            sample_rate: self.sample_rate,
            amplitude: self.amplitude,
            frequency: self.frequency,
            offset: Box::new(offset),
            phase: self.phase,
        })
    }
}

impl Signal for Sine {
    fn evaluate(&mut self) -> f64 {
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