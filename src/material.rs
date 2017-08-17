use interaction::Interaction;
use reflection::BSDF;

pub trait Material : Sync + Send {
    fn compute_scattering(&self, i: &Interaction) -> BSDF;
}
