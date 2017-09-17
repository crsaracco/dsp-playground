//! Signal generators.

// Sine wave generator
pub mod sine;
pub use self::sine::Sine;

// Saw wave generator
pub mod saw;
pub use self::saw::Saw;