use common::PI;
use reflection::{BxDF, BxDFFlag, REFLECTION, DIFFUSE, abs_cos_theta, same_hemisphere};
use sampling::cosine_sample_hemisphere;
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

/// Perfect diffuse reflection.
pub struct LambertianReflectionBRDF {
    r: Spectrum,
}

impl LambertianReflectionBRDF {
    pub fn new(r: Spectrum) -> LambertianReflectionBRDF {
        LambertianReflectionBRDF { r }
    }
}

impl BxDF for LambertianReflectionBRDF {
    fn flag(&self) -> BxDFFlag {
        REFLECTION | DIFFUSE
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        self.r / PI
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        let mut wi = cosine_sample_hemisphere(sample);
        // flip wi to make sure that wi and wo are in the same hemisphere
        if wo.z < 0. {
            wi.z *= -1.;
        }
        return (wi, self.f(wo, wi), self.pdf(wo, wi));
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        if same_hemisphere(wo, wi) {
            abs_cos_theta(wi) / PI
        } else {
            0.
        }
    }
}
