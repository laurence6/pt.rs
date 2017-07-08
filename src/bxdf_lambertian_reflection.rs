use common::PI;
use reflection::{BxDF, BxDFFlag, REFLECTION, DIFFUSE};
use spectrum::Spectrum;
use vector::Vector3f;

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
}
