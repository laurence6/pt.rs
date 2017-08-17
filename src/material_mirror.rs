use std::sync::Arc;

use bxdf_specular_reflection::SpecularReflectionBRDF;
use common::INFINITY;
use fresnel::FresnelNoOp;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::Texture;

pub struct MirrorMaterial {
    kr: Arc<Texture>,
}

impl MirrorMaterial {
    pub fn new(kr: Arc<Texture>) -> MirrorMaterial {
        MirrorMaterial { kr }
    }
}

impl Material for MirrorMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(1., i);
        let r = self.kr.evaluate(i).clamp(0., INFINITY);
        if !r.is_black() {
            bsdf.add(Box::new(SpecularReflectionBRDF::new(r, Box::new(FresnelNoOp::new()))));
        }
        return bsdf;
    }
}
