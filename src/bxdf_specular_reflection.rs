use fresnel::Fresnel;
use reflection::{BxDF, BxDFFlag, REFLECTION, SPECULAR, cos_theta, abs_cos_theta};
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

pub struct SpecularReflectionBRDF {
    r: Spectrum,
    fresnel: Box<Fresnel>,
}

impl SpecularReflectionBRDF {
    pub fn new(r: Spectrum, fresnel: Box<Fresnel>) -> SpecularReflectionBRDF {
        SpecularReflectionBRDF { r, fresnel }
    }
}

impl BxDF for SpecularReflectionBRDF {
    fn flag(&self) -> BxDFFlag {
        REFLECTION | SPECULAR
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        Spectrum::default()
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        0.
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        let wi = Vector3f::new(-wo.x, -wo.y, wo.z);
        let pdf = 1.;
        let f = self.fresnel.evaluate(cos_theta(wi)) * self.r / abs_cos_theta(wi);
        return (wi, f, pdf);
    }
}
