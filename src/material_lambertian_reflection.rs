use std::rc::Rc;

use bxdf_lambertian_reflection::LambertianReflectionBRDF;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::Texture;

pub struct LambertianReflectionMaterial {
    kd: Rc<Texture>,
}

impl LambertianReflectionMaterial {
    pub fn new(kd: Rc<Texture>) -> LambertianReflectionMaterial {
        LambertianReflectionMaterial { kd }
    }
}

impl Material for LambertianReflectionMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let r = self.kd.evaluate(i);
        let mut bsdf = BSDF::new(1., i);
        if !r.is_black() {
            bsdf.add(Box::new(LambertianReflectionBRDF::new(r)));
        }
        return bsdf;
    }
}
