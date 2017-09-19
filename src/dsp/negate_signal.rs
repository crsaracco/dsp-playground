//! Signal that negates the passed-in signal argument

use dsp::traits::Signal;

/// NegateSignal struct
pub struct NegateSignal {
    negated: Box<Signal>,
}

impl NegateSignal {
    /// Creates a new NegateSignal signal
    pub fn new(negated: Box<Signal>) -> NegateSignal {
        NegateSignal {negated}
    }
}

impl Signal for NegateSignal {
    fn evaluate(&mut self) -> f64 {
        -1.0 * self.negated.evaluate()
    }
}