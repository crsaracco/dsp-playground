use dsp::evaluatable::Evaluatable;

pub struct AddSignals {
    added: Vec<Box<Evaluatable>>,
}

impl AddSignals {
    pub fn new(added: Vec<Box<Evaluatable>>) -> AddSignals {
        AddSignals {added}
    }
}

impl Evaluatable for AddSignals {
    fn evaluate(&mut self) -> (f32, f32) {
        let mut left: f32 = 0.0;
        let mut right: f32 = 0.0;
        for c in &mut self.added {
            let output = c.evaluate();
            left += output.0;
            right += output.1;
        }

        (left, right)
    }
}