use evaluatable::Evaluatable;

pub struct Combine<E>
    where E: Evaluatable
{
    combined: Vec<E>,
}

impl<E> Combine<E>
    where E: Evaluatable
{
    pub fn new(combined: Vec<E>) -> Combine<E> {
        Combine {combined}
    }
}

impl<E> Evaluatable for Combine<E>
    where E: Evaluatable
{
    fn evaluate(&mut self) -> (f32, f32) {
        let mut left: f32 = 0.0;
        let mut right: f32 = 0.0;
        for c in &mut self.combined {
            let output = c.evaluate();
            left += output.0;
            right += output.1;
        }

        (left, right)
    }
}