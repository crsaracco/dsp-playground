//! Signal generators.

// Sine wave generator
pub mod sine;
pub use self::sine::Sine;

// Saw wave generator
pub mod saw;
pub use self::saw::Saw;

// Square wave generator
pub mod square;
pub use self::square::Square;

// Triangle wave generator
pub mod triangle;
pub use self::triangle::Triangle;