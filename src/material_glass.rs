use std::sync::Arc;

use bxdf_specular_reflection::SpecularReflectionBRDF;
use bxdf_specular_transmission::SpecularTransmissionBTDF;
use common::INFINITY;
use fresnel::FresnelDielectric;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use texture::{TextureFloat, TextureSpectrum};

pub struct GlassMaterial {
    kr: Arc<TextureSpectrum>,
    kt: Arc<TextureSpectrum>,
    eta: Arc<TextureFloat>,
}

impl GlassMaterial {
    pub fn new(kr: Arc<TextureSpectrum>, kt: Arc<TextureSpectrum>, eta: Arc<TextureFloat>) -> GlassMaterial {
        GlassMaterial { kr, kt, eta }
    }
}

impl Material for GlassMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let eta = self.eta.evaluate(i);
        let mut bsdf = BSDF::new(eta, i);

        let r = self.kr.evaluate(i).clamp(0., INFINITY);
        let t = self.kt.evaluate(i).clamp(0., INFINITY);

        if !r.is_black() {
            let fresnel = FresnelDielectric::new(1., eta);
            bsdf.add(SpecularReflectionBRDF::new(r, fresnel));
        }
        if !t.is_black() {
            bsdf.add(SpecularTransmissionBTDF::new(t, 1., eta));
        }

        return bsdf;
    }
}
