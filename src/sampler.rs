use common::Float;

pub trait Sampler {
    fn SamplesPerPixel(&self) -> i64;
    fn StartPixel(&self);
}
