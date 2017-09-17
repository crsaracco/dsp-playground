use evaluatable::Evaluatable;

pub struct AddSignals<E>
    where E: Evaluatable
{
    added: Vec<E>,
}

impl<E> AddSignals<E>
    where E: Evaluatable
{
    pub fn new(added: Vec<E>) -> AddSignals<E> {
        AddSignals {added}
    }
}

impl<E> Evaluatable for AddSignals<E>
    where E: Evaluatable
{
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