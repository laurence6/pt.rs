use sampling::cosine_sample_hemisphere;
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

pub type BxDFType = u8;
pub const REFLECTION:   BxDFType = 1 << 0;
pub const TRANSMISSION: BxDFType = 1 << 1;
pub const DIFFUSE:      BxDFType = 1 << 2;
pub const GLOSSY:       BxDFType = 1 << 3;
pub const SPECULAR:     BxDFType = 1 << 4;
pub const ALL:          BxDFType = REFLECTION | TRANSMISSION | DIFFUSE | GLOSSY | SPECULAR;

pub trait BxDF {
    /// Return BxDF type.
    fn bxdf_type(&self) -> BxDFType;

    /// Return value of distribution function for the given pair of direction.
    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum;

    /// Return the direction of incident ray, value of distribution function.
    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum) {
        let mut wi = cosine_sample_hemisphere(sample);
        // flip wi to make sure that wi and wo are in the same hemisphere
        if wo.z < 0. {
            wi.z *= -1.;
        }
        return (wi, self.f(wo, wi));
    }
}

trait BxDFMatchType {
    fn match_type(&self, t: BxDFType) -> bool;
}

impl<T> BxDFMatchType for T where T: BxDF {
    fn match_type(&self, t: BxDFType) -> bool {
        let bxdf_type = self.bxdf_type();
        debug_assert!(bxdf_type <= ALL);
        return (bxdf_type & t) == bxdf_type;
    }
}
