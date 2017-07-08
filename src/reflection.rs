use sampling::cosine_sample_hemisphere;
use spectrum::Spectrum;
use vector::{Vector3f, Point2f};

pub type BxDFFlag = u8;
pub const REFLECTION:   BxDFFlag = 1 << 0;
pub const TRANSMISSION: BxDFFlag = 1 << 1;
pub const DIFFUSE:      BxDFFlag = 1 << 2;
pub const GLOSSY:       BxDFFlag = 1 << 3;
pub const SPECULAR:     BxDFFlag = 1 << 4;
pub const ALL:          BxDFFlag = REFLECTION | TRANSMISSION | DIFFUSE | GLOSSY | SPECULAR;

pub trait BxDF {
    /// Return BxDF flag.
    fn flag(&self) -> BxDFFlag;

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

    fn has_flag(&self, t: BxDFFlag) -> bool {
        let flag = self.flag();
        debug_assert!(flag <= ALL);
        return (t & flag) == t;
    }
}
