use fresnel::{Fresnel, FresnelDielectric};
use reflection::{BxDF, BxDFFlag, TRANSMISSION, SPECULAR, cos_theta, abs_cos_theta, refract};
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

pub struct SpecularTransmissionBTDF {
    t: Spectrum,
    eta_a: f32, // index of refraction above surface
    eta_b: f32,
    fresnel: FresnelDielectric,
}

impl SpecularTransmissionBTDF {
    pub fn new(t: Spectrum, eta_a: f32, eta_b: f32) -> SpecularTransmissionBTDF {
        SpecularTransmissionBTDF {
            t,
            eta_a,
            eta_b,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl BxDF for SpecularTransmissionBTDF {
    fn flag(&self) -> BxDFFlag {
        TRANSMISSION | SPECULAR
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        Spectrum::default()
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        let (eta_i, eta_t) = if cos_theta(wo) > 0. {
            (self.eta_a, self.eta_b)
        } else {
            (self.eta_b, self.eta_a)
        };

        let n = if wo.z > 0. {
            Vector3f::new(0., 0., 1.)
        } else {
            Vector3f::new(0., 0., -1.)
        };
        let wi = refract(wo, n, eta_i / eta_t);
        if wi.is_none() {
            return Default::default();
        }
        let wi = wi.unwrap();

        let pdf = 1.;

        let f = self.t * (Spectrum::from(1.) - self.fresnel.evaluate(cos_theta(wi))) / abs_cos_theta(wi);
        let f = f * (eta_i * eta_i) / (eta_t * eta_t);
        return (wi, f, pdf)
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        0.
    }
}
