//! Signal that negates the passed-in signal argument

use dsp::evaluatable::Evaluatable;

/// NegateSignal struct
pub struct NegateSignal {
    negated: Box<Evaluatable>,
}

impl NegateSignal {
    /// Creates a new NegateSignal signal
    pub fn new(negated: Box<Evaluatable>) -> NegateSignal {
        NegateSignal {negated}
    }
}

impl Evaluatable for NegateSignal {
    fn evaluate(&mut self) -> (f32, f32) {
        let (left, right) = self.negated.evaluate();

        (-left, -right)
    }
}