use std::sync::Arc;

use bxdf_lambertian_reflection::LambertianReflectionBRDF;
use common::INFINITY;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::TextureSpectrum;

pub struct LambertianReflectionMaterial {
    kd: Arc<TextureSpectrum>,
}

impl LambertianReflectionMaterial {
    pub fn new(kd: Arc<TextureSpectrum>) -> LambertianReflectionMaterial {
        LambertianReflectionMaterial { kd }
    }
}

impl Material for LambertianReflectionMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(1., i);
        let r = self.kd.evaluate(i).clamp(0., INFINITY);
        if !r.is_black() {
            bsdf.add(LambertianReflectionBRDF::new(r));
        }
        return bsdf;
    }
}
