use std::sync::Arc;

use bxdf_lambertian_reflection::LambertianReflectionBRDF;
use common::INFINITY;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::Texture;

pub struct LambertianReflectionMaterial {
    kd: Arc<Texture>,
}

impl LambertianReflectionMaterial {
    pub fn new(kd: Arc<Texture>) -> LambertianReflectionMaterial {
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
