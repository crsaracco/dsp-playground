//! Signal that negates the passed-in signal argument

use dsp::evaluatable::Evaluatable;

/// NegateSignal struct
pub struct NegateSignal<E>
    where E: Evaluatable
{
    negated: E,
}

impl<E> NegateSignal<E>
    where E: Evaluatable
{
    /// Creates a new NegateSignal signal
    pub fn new(negated: E) -> NegateSignal<E> {
        NegateSignal {negated}
    }
}

impl<E> Evaluatable for NegateSignal<E>
    where E: Evaluatable
{
    fn evaluate(&mut self) -> (f32, f32) {
        let (left, right) = self.negated.evaluate();

        (-left, -right)
    }
}