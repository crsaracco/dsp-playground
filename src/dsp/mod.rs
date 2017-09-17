//! DSP library.
//!
//! Provides:
//!  - Waveform generators (sine, saw) that generates the corresponding signal
//!  - The ability to negate a signal
//!  - The ability to add signals together
//!  - A trait called "Evaluatable" which all signals must use (might rename this to "Signal")

pub mod generators;
pub mod add_signals;
pub mod negate_signal;
pub mod evaluatable;