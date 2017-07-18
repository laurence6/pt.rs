use interaction::Interaction;
use reflection::BSDF;

pub trait Material {
    fn compute_scattering(&self, i: &Interaction) -> BSDF;
}
