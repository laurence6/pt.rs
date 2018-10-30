use std::sync::Arc;

use bxdf_specular_reflection::SpecularReflectionBRDF;
use common::INFINITY;
use fresnel::FresnelNoOp;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::TextureSpectrum;

pub struct MirrorMaterial {
    kr: Arc<TextureSpectrum>,
}

impl MirrorMaterial {
    pub fn new(kr: Arc<TextureSpectrum>) -> MirrorMaterial {
        MirrorMaterial { kr }
    }
}

impl Material for MirrorMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(i);
        let r = self.kr.evaluate(i).clamp(0., INFINITY);
        if !r.is_black() {
            bsdf.add(SpecularReflectionBRDF::new(r, FresnelNoOp::new()));
        }
        return bsdf;
    }
}
