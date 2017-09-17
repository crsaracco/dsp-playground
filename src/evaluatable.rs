pub trait Evaluatable {
    fn evaluate(&mut self) -> (f32, f32);
}