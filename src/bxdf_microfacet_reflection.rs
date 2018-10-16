use fresnel::Fresnel;
use microfacet::MicrofacetDistribution;
use reflection::{BxDF, BxDFFlag, REFLECTION, GLOSSY, abs_cos_theta, same_hemisphere, reflect};
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

/// Torrance–Sparrow(Cook–Torrance) model.
pub struct MicrofacetReflectionBRDF<T> where T: Fresnel {
    r: Spectrum,
    distribution: MicrofacetDistribution,
    fresnel: T,
}

impl<T> MicrofacetReflectionBRDF<T> where T: Fresnel {
    pub fn new(r: Spectrum, distribution: MicrofacetDistribution, fresnel: T) -> MicrofacetReflectionBRDF<T> {
        MicrofacetReflectionBRDF { r, distribution, fresnel }
    }
}

impl<T> BxDF for MicrofacetReflectionBRDF<T> where T: Fresnel {
    fn flag(&self) -> BxDFFlag {
        REFLECTION | GLOSSY
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        let cos_theta_o = abs_cos_theta(wo);
        let cos_theta_i = abs_cos_theta(wi);
        if cos_theta_o == 0. || cos_theta_i == 0. {
            return Spectrum::default();
        }

        let wh = wo + wi;
        if wh.x == 0. && wh.y == 0. && wh.z == 0. {
            return Spectrum::default();
        }

        let wh = wh.normalize();

        let d = self.distribution.d(wh);
        let g = self.distribution.g(wo, wi);
        let f = self.fresnel.evaluate(wh.dot(wi));
        return self.r * d * g * f
            / (4. * cos_theta_o * cos_theta_i);
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        if wo.z == 0. {
            return Default::default();
        }

        let wh = self.distribution.sample_wh(wo, sample);
        let wi = reflect(wo, wh);
        if !same_hemisphere(wo, wi) {
            return Default::default();
        }

        let f = self.f(wo, wi);
        let pdf = self.distribution.pdf(wo, wh) / (wh.dot(wo) * 4.);

        return (wi, f, pdf);
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        if !same_hemisphere(wo, wi) {
            return 0.;
        }
        let wh = (wo + wi).normalize();
        return self.distribution.pdf(wo, wh) / (wh.dot(wo) * 4.);
    }
}
