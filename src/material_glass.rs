use std::sync::Arc;

use bxdf_microfacet_reflection::MicrofacetReflectionBRDF;
use bxdf_microfacet_transmission::MicrofacetTransmissionBTDF;
use bxdf_specular_reflection::SpecularReflectionBRDF;
use bxdf_specular_transmission::SpecularTransmissionBTDF;
use common::INFINITY;
use fresnel::FresnelDielectric;
use interaction::Interaction;
use material::Material;
use microfacet::MicrofacetDistribution;
use reflection::BSDF;
use texture::{TextureFloat, TextureSpectrum};

pub struct GlassMaterial {
    kr: Arc<TextureSpectrum>,
    kt: Arc<TextureSpectrum>,
    eta: Arc<TextureFloat>,
    roughness_u: Arc<TextureFloat>,
    roughness_v: Arc<TextureFloat>,
}

impl GlassMaterial {
    pub fn new(kr: Arc<TextureSpectrum>, kt: Arc<TextureSpectrum>, eta: Arc<TextureFloat>, roughness_u: Arc<TextureFloat>, roughness_v: Arc<TextureFloat>) -> GlassMaterial {
        GlassMaterial { kr, kt, eta, roughness_u, roughness_v }
    }
}

impl Material for GlassMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let eta = self.eta.evaluate(i);
        let mut bsdf = BSDF::new(eta, i);

        let r = self.kr.evaluate(i).clamp(0., INFINITY);
        let t = self.kt.evaluate(i).clamp(0., INFINITY);
        let rough_u = self.roughness_u.evaluate(i);
        let rough_v = self.roughness_v.evaluate(i);

        if rough_u == 0. && rough_v == 0. {
            if !r.is_black() {
                let fresnel = FresnelDielectric::new(1., eta);
                bsdf.add(SpecularReflectionBRDF::new(r, fresnel));
            }
            if !t.is_black() {
                bsdf.add(SpecularTransmissionBTDF::new(t, 1., eta));
            }
        } else {
            let distrib = MicrofacetDistribution::new(rough_u, rough_v);
            if !r.is_black() {
                let fresnel = FresnelDielectric::new(1., eta);
                bsdf.add(MicrofacetReflectionBRDF::new(r, distrib, fresnel));
            }
            if !t.is_black() {
                bsdf.add(MicrofacetTransmissionBTDF::new(t, 1., eta, distrib));
            }
        }

        return bsdf;
    }
}
