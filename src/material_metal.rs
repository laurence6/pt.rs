use std::sync::Arc;

use bxdf_microfacet_reflection::MicrofacetReflectionBRDF;
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

        let r = self.r.evaluate(i);

        let rough_u = self.roughness_u.evaluate(i);
        let rough_v = self.roughness_v.evaluate(i);
        let distrib = MicrofacetDistribution::new(rough_u, rough_v);

        let eta = self.eta.evaluate(i);
        let k = self.k.evaluate(i);
        let fresnel = FresnelConductor::new(Spectrum::from(1.), eta, k);

        bsdf.add(MicrofacetReflectionBRDF::new(r, distrib, fresnel));

        return bsdf;
    }
}
