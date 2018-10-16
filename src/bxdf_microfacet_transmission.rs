use fresnel::{Fresnel, FresnelDielectric};
use microfacet::MicrofacetDistribution;
use reflection::{BxDF, BxDFFlag, TRANSMISSION, GLOSSY, cos_theta, same_hemisphere, refract};
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

/// Torrance–Sparrow(Cook–Torrance) model.
pub struct MicrofacetTransmissionBTDF {
    t: Spectrum,
    eta_a: f32, // index of refraction above surface
    eta_b: f32,
    distribution: MicrofacetDistribution,
    fresnel: FresnelDielectric,
}

impl MicrofacetTransmissionBTDF {
    pub fn new(t: Spectrum, eta_a: f32, eta_b: f32, distribution: MicrofacetDistribution) -> MicrofacetTransmissionBTDF {
        MicrofacetTransmissionBTDF {
            t,
            eta_a,
            eta_b,
            distribution,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl BxDF for MicrofacetTransmissionBTDF {
    fn flag(&self) -> BxDFFlag {
        TRANSMISSION | GLOSSY
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        if same_hemisphere(wo, wi) {
            return Spectrum::default();
        }

        let cos_theta_o = cos_theta(wo);
        let cos_theta_i = cos_theta(wi);
        if cos_theta_o == 0. || cos_theta_i == 0. {
            return Spectrum::default();
        }

        let eta = if cos_theta(wo) > 0. {
            self.eta_b / self.eta_a
        } else {
            self.eta_a / self.eta_b
        };
        let mut wh = (wo + wi * eta).normalize();
        if wh.z < 0. {
            wh *= -1.;
        }

        let d = self.distribution.d(wh);
        let g = self.distribution.g(wo, wi);
        let f = self.fresnel.evaluate(wh.dot(wo));
        return (self.t * d * g * (Spectrum::from(1.) - f) * eta * eta * wi.dot(wh).abs() * wo.dot(wh).abs()
            / ((wo.dot(wh) + wi.dot(wh) * eta).powi(2) * cos_theta_o * cos_theta_i)).abs();
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        if wo.z == 0. {
            return Default::default();
        }

        let wh = self.distribution.sample_wh(wo, sample);
        let eta = if cos_theta(wo) > 0. {
            self.eta_a / self.eta_b
        } else {
            self.eta_b / self.eta_a
        };
        let wi = refract(wo, wh, eta);
        if wi.is_none() {
            return Default::default();
        }
        let wi = wi.unwrap();

        let f = self.f(wo, wi);
        let pdf = self.pdf(wo, wi);

        return (wi, f, pdf);
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        if same_hemisphere(wo, wi) {
            return 0.;
        }

        let eta = if cos_theta(wo) > 0. {
            self.eta_b / self.eta_a
        } else {
            self.eta_a / self.eta_b
        };
        let wh = (wo + wi * eta).normalize();

        let sqrt_denom = wo.dot(wh) + wi.dot(wh) * eta;
        let dwh_dwi = (wi.dot(wh) * eta * eta / sqrt_denom.powi(2)).abs();
        return self.distribution.pdf(wo, wh) * dwh_dwi;
    }
}
