//! Parametric signal generators.
//!
//! Accept Evaluatable inputs for frequency, amplitude, etc.

// Parametric sine wave generator
pub mod sine;
pub use self::sine::Sine;