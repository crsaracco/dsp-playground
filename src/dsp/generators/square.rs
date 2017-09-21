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

use dsp::traits::Signal;

/// Square wave generator struct.
pub struct Square<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    sample_rate: f64,  // Sample rate (for audio playback, etc) - Should be the same throughout the whole project
    amplitude: A,      // Amplitude of the Square wave
    frequency: F,      // Frequency of the Square wave
    offset: O,         // DC offset of the Square wave    (+/- y axis)
    phase: f64,        // Phase offset of the Square wave (+/- x axis, as a percent of the whole period)
}

impl<A, F, O> Square<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    /// Creates a new Square wave signal generator.
    pub fn new(amplitude: A, frequency: F, offset: O) -> Square<A, F, O> {
        Square {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl<A, F, O> Signal for Square<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    fn evaluate(&mut self) -> (f64) {
        let amplitude = self.amplitude.evaluate();
        let frequency = self.frequency.evaluate();
        let offset = self.offset.evaluate();

        let mut output = match self.phase {
            n if n <= 0.5 => -1.0,
            _ => 1.0,
        };

        self.phase = (self.phase + frequency / self.sample_rate).fract();

        output *= amplitude;
        output += offset;

        output
    }
}