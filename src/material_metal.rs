use std::sync::Arc;

use bxdf_specular_reflection::SpecularReflectionBRDF;
use fresnel::FresnelConductor;
use interaction::Interaction;
use material::Material;
use reflection::BSDF;
use spectrum::Spectrum;
use texture::TextureSpectrum;

pub struct MetalMaterial {
    r: Arc<TextureSpectrum>,
    eta: Arc<TextureSpectrum>,
    k: Arc<TextureSpectrum>,
}

impl MetalMaterial {
    pub fn new(r: Arc<TextureSpectrum>, eta: Arc<TextureSpectrum>, k: Arc<TextureSpectrum>) -> MetalMaterial {
        MetalMaterial { r, eta, k }
    }
}

impl Material for MetalMaterial {
    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        let mut bsdf = BSDF::new(1., i);
        let r = self.r.evaluate(i);
        let eta = self.eta.evaluate(i);
        let k = self.k.evaluate(i);
        let fresnel = FresnelConductor::new(Spectrum::from(1.), eta, k);
        let brdf = SpecularReflectionBRDF::new(r, fresnel);
        bsdf.add(brdf);
        return bsdf;
    }
}
