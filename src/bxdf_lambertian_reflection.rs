use common::PI;
use reflection::{REFLECTION, DIFFUSE, BxDFType, BxDF};
use spectrum::Spectrum;
use vector::Vector3f;

/// Perfect diffuse reflection.
pub struct LambertianReflectionBRDF {
    r: Spectrum,
}

impl LambertianReflectionBRDF {
    pub fn new(r: Spectrum) -> LambertianReflectionBRDF {
        LambertianReflectionBRDF { r: r }
    }
}

impl BxDF for LambertianReflectionBRDF {
    fn bxdf_type(&self) -> BxDFType {
        REFLECTION | DIFFUSE
    }

    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        self.r / PI
    }
}
