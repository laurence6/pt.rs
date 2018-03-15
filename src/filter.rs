use vector::Point2f;

pub trait Filter {
    fn radius(&self) -> f32;

    /// A point relative to the center of Filter.
    fn evaluate(&self, p: Point2f) -> f32;
}
