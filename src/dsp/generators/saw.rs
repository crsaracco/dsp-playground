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

use dsp::traits::Signal;

/// Saw wave generator struct.
pub struct Saw<A, F, O> where
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

impl<A, F, O> Saw<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    /// Creates a new Saw wave signal generator.
    pub fn new(amplitude: A, frequency: F, offset: O) -> Saw<A, F, O> {
        Saw {
            sample_rate: 44100.0,
            amplitude,
            frequency,
            offset,
            phase: 0.0,
        }
    }
}

impl<A, F, O> Signal for Saw<A, F, O> where
    A: Signal,
    F: Signal,
    O: Signal,
{
    fn evaluate(&mut self) -> (f64) {
        let amplitude = self.amplitude.evaluate();
        let frequency = self.frequency.evaluate();
        let offset = self.offset.evaluate();

        let mut output = self.phase * 2.0 - 1.0;
        let last_phase = self.phase;
        self.phase = (self.phase + frequency / self.sample_rate).fract();

        output *= amplitude;
        output += offset;

        //println!("{}", output);

        output
    }
}