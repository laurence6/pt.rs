use std::sync::Arc;

use bxdf_lambertian_reflection::LambertianReflectionBRDF;
use bxdf_microfacet_reflection::MicrofacetReflectionBRDF;
use bxdf_specular_reflection::SpecularReflectionBRDF;
use common::INFINITY;
use fresnel::FresnelDielectric;
use interaction::Interaction;
use material::Material;
use microfacet::MicrofacetDistribution;
use reflection::BSDF;
use texture::{TextureFloat, TextureSpectrum};

pub struct PlasticMaterial {
    kd: Arc<TextureSpectrum>, // diffuse
    ks: Arc<TextureSpectrum>, // glossy
    roughness: Arc<TextureFloat>,
}

impl PlasticMaterial {
    pub fn new(kd: Arc<TextureSpectrum>, ks: Arc<TextureSpectrum>, roughness: Arc<TextureFloat>) -> PlasticMaterial {
        PlasticMaterial { kd, ks, roughness }
    }
}

impl Material for PlasticMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(i);

        let d = self.kd.evaluate(i).clamp(0., INFINITY);
        let s = self.ks.evaluate(i).clamp(0., INFINITY);

        if !d.is_black() {
            bsdf.add(LambertianReflectionBRDF::new(d));
        }

        if !s.is_black() {
            let fresnel = FresnelDielectric::new(1.5, 1.);
            let rough = self.roughness.evaluate(i);
            if rough == 0. {
                bsdf.add(SpecularReflectionBRDF::new(s, fresnel));
            } else {
                let distrib = MicrofacetDistribution::new(rough, rough);
                bsdf.add(MicrofacetReflectionBRDF::new(s, distrib, fresnel));
            }
        }

        return bsdf;
    }
}
