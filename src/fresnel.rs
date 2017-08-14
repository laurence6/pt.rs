use spectrum::Spectrum;

pub trait Fresnel {
    fn evaluate(&self, cos_i: f32) -> Spectrum;
}
