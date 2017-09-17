use evaluatable::Evaluatable;

pub struct Combine<F, S>
    where
        F: Evaluatable,
        S: Evaluatable,
{
    first: F,
    second: S,
}

impl<F, S> Combine<F, S>
    where
        F: Evaluatable,
        S: Evaluatable,
{
    pub fn new(first: F, second: S) -> Combine<F, S> {
        Combine {first, second}
    }
}

impl<F, S> Evaluatable for Combine<F, S>
    where
        F: Evaluatable,
        S: Evaluatable,
{
    fn evaluate(&mut self) -> (f32, f32) {
        let first_output = self.first.evaluate();
        let second_output = self.second.evaluate();


        (first_output.0 + second_output.0, first_output.1 + second_output.1)
    }
}