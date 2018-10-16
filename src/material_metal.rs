use std::sync::Arc;

use bxdf_microfacet_reflection::MicrofacetReflectionBRDF;
use bxdf_specular_reflection::SpecularReflectionBRDF;
use common::INFINITY;
use fresnel::FresnelConductor;
use interaction::Interaction;
use material::Material;
use microfacet::MicrofacetDistribution;
use reflection::BSDF;
use spectrum::Spectrum;
use texture::{TextureFloat, TextureSpectrum};

pub struct MetalMaterial {
    r: Arc<TextureSpectrum>,
    eta: Arc<TextureSpectrum>,
    k: Arc<TextureSpectrum>,
    roughness_u: Arc<TextureFloat>,
    roughness_v: Arc<TextureFloat>,
}

impl MetalMaterial {
    pub fn new(r: Arc<TextureSpectrum>, eta: Arc<TextureSpectrum>, k: Arc<TextureSpectrum>, roughness_u: Arc<TextureFloat>, roughness_v: Arc<TextureFloat>) -> MetalMaterial {
        MetalMaterial { r, eta, k, roughness_u, roughness_v }
    }
}

impl Material for MetalMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(1., i);

        let r = self.r.evaluate(i).clamp(0., INFINITY);
        let eta = self.eta.evaluate(i);
        let k = self.k.evaluate(i);
        let rough_u = self.roughness_u.evaluate(i);
        let rough_v = self.roughness_v.evaluate(i);

        let fresnel = FresnelConductor::new(Spectrum::from(1.), eta, k);

        if rough_u == 0. && rough_v == 0. {
            bsdf.add(SpecularReflectionBRDF::new(r, fresnel));
        } else {
            let distrib = MicrofacetDistribution::new(rough_u, rough_v);
            bsdf.add(MicrofacetReflectionBRDF::new(r, distrib, fresnel));
        }

        return bsdf;
    }
}
