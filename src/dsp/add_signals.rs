//! Signal that takes multiple other signals and adds them together

use dsp::traits::Signal;

/// AddSignals struct
pub struct AddSignals {
    added: Vec<Box<Signal>>,
}

impl AddSignals {
    /// Creates a new AddSignals signal
    pub fn new(added: Vec<Box<Signal>>) -> AddSignals {
        AddSignals {added}
    }
}

impl Signal for AddSignals {
    fn evaluate(&mut self) -> f64 {
        let mut output: f64 = 0.0;

        for c in &mut self.added {
            output += c.evaluate();
        }

        output
    }
}