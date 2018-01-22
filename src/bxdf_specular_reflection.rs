use fresnel::Fresnel;
use reflection::{BxDF, BxDFFlag, REFLECTION, SPECULAR, cos_theta, abs_cos_theta};
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

pub struct SpecularReflectionBRDF<T> where T: Fresnel {
    r: Spectrum,
    fresnel: T,
}

impl<T> SpecularReflectionBRDF<T> where T: Fresnel {
    pub fn new(r: Spectrum, fresnel: T) -> SpecularReflectionBRDF<T> {
        SpecularReflectionBRDF { r, fresnel }
    }
}

impl<T> BxDF for SpecularReflectionBRDF<T> where T: Fresnel {
    fn flag(&self) -> BxDFFlag {
        REFLECTION | SPECULAR
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        Spectrum::default()
    }

    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        let wi = Vector3f::new(-wo.x, -wo.y, wo.z);
        let pdf = 1.;
        let f = self.r * self.fresnel.evaluate(cos_theta(wi)) / abs_cos_theta(wi);
        return (wi, f, pdf);
    }

    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        0.
    }
}
