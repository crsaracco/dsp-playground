//! Signal generators.

// Parametric signal generators (accept Evaluatable inputs for frequency, amplitude, etc.)
pub mod parametric;

// Sine wave generator
pub mod sine;
pub use self::sine::Sine;

// Saw wave generator
pub mod saw;
pub use self::saw::Saw;

// Square wave generator
pub mod square;
pub use self::square::Square;