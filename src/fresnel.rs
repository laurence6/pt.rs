use spectrum::Spectrum;

pub trait Fresnel {
    fn evaluate(&self, cos_i: f32) -> Spectrum;
}

pub struct FresnelNoOp {}

impl FresnelNoOp {
    pub fn new() -> FresnelNoOp {
        FresnelNoOp {}
    }
}

impl Fresnel for FresnelNoOp {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        Spectrum::from(1.)
    }
}
