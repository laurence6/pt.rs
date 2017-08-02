use common::PI;
use interaction::Interaction;
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

    fn has_flag(&self, t: BxDFFlag) -> bool {
        let flag = self.flag();
        debug_assert!(flag <= ALL);
        return (t & flag) == t;
    }

    /// Return value of distribution function for the given pair of direction.
    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum;

    /// Return pdf for the given pair of direction.
    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        if same_hemisphere(wo, wi) {
            abs_cos_theta(wi) / PI
        } else {
            0.
        }
    }

    /// Return the direction of incident ray, value of distribution function, pdf.
    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
        let mut wi = cosine_sample_hemisphere(sample);
        // flip wi to make sure that wi and wo are in the same hemisphere
        if wo.z < 0. {
            wi.z *= -1.;
        }
        return (wi, self.f(wo, wi), self.pdf(wo, wi));
    }
}

pub struct BSDF {
    bxdfs: Vec<Box<BxDF>>,
    eta: f32,
    n: Vector3f,
    s: Vector3f,
    t: Vector3f,
}

impl BSDF {
    /// Relative index of refraction.
    pub fn new(eta: f32, i: &Interaction) -> BSDF {
        let n = Vector3f::from(i.n);
        let s = i.dpdu.normalize();
        let t = n.cross(s);
        return BSDF {
            bxdfs: Vec::new(),
            eta,
            n,
            s,
            t,
        };
    }

    /// Add a BxDF component.
    pub fn add(&mut self, bxdf: Box<BxDF>) {
        self.bxdfs.push(bxdf);
    }

    fn world_to_local(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            v.dot(self.s),
            v.dot(self.t),
            v.dot(self.n),
        )
    }

    fn local_to_world(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.s.x * v.x + self.t.x * v.y + self.n.x * v.z,
            self.s.y * v.x + self.t.y * v.y + self.n.y * v.z,
            self.s.z * v.x + self.t.z * v.y + self.n.z * v.z,
        )
    }

    pub fn f(&self, wo_w: Vector3f, wi_w: Vector3f) -> Spectrum {
        let wo = self.world_to_local(wo_w);
        if wo.z == 0. {
            return Default::default();
        }
        let wi = self.world_to_local(wi_w);

        let reflect = wi_w.dot(self.n) * wo_w.dot(self.n) > 0.;

        let mut f = Spectrum::default();
        for bxdf in self.bxdfs.iter() {
            if (reflect && bxdf.has_flag(REFLECTION))
                || (!reflect && bxdf.has_flag(TRANSMISSION)) {
                f += bxdf.f(wo, wi);
            }
        }

        return f;
    }

    /// Return the direction of incident ray, value of distribution function, pdf, flag of the chosen BxDF.
    pub fn sample_f(&self, wo_w: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32, BxDFFlag) {
        let n_bxdfs = self.bxdfs.len();
        if n_bxdfs == 0 {
            return Default::default();
        }

        let n_bxdf_f = sample[0] * n_bxdfs as f32;
        let n_bxdf = n_bxdf_f.floor() as usize;
        let bxdf = &self.bxdfs[n_bxdf];

        // remap sample to [0, 1)
        let sample = Point2f::new(n_bxdf_f - n_bxdf as f32, sample[1]);

        let wo = self.world_to_local(wo_w);
        if wo.z == 0. {
            return Default::default();
        }
        let (wi, mut f, mut pdf) = bxdf.sample_f(wo, sample);
        if pdf == 0. {
            return Default::default();
        }
        let wi_w = self.local_to_world(wi);

        if !bxdf.has_flag(SPECULAR) {
            for i in 0..n_bxdfs {
                if i != n_bxdf {
                    pdf += self.bxdfs[i].pdf(wo, wi);
                }
            }
            if n_bxdfs > 1 {
                pdf /= n_bxdfs as f32;
            }

            let reflect = wi_w.dot(self.n) * wo_w.dot(self.n) > 0.;
            f = Spectrum::default();
            for bxdf in self.bxdfs.iter() {
                if (reflect && bxdf.has_flag(REFLECTION)) || (!reflect && bxdf.has_flag(TRANSMISSION)) {
                    f += bxdf.f(wo, wi);
                }
            }
        }

        return (wi_w, f, pdf, bxdf.flag());
    }
}

fn abs_cos_theta(w: Vector3f) -> f32 {
    w.z.abs()
}

fn same_hemisphere(w1: Vector3f, w2: Vector3f) -> bool {
    w1.z * w2.z > 0.
}

#[cfg(test)]
mod test {
    use common::EPSILON;
    use interaction::Interaction;
    use reflection::{BxDF, BSDF, BxDFFlag, REFLECTION, TRANSMISSION, DIFFUSE, GLOSSY, SPECULAR};
    use spectrum::Spectrum;
    use vector::{Vector3f, Normal3f, Point3f};

    #[test]
    fn test_has_type() {
        struct TestBxDF {}
        impl BxDF for TestBxDF {
            fn flag(&self) -> BxDFFlag {
                REFLECTION | DIFFUSE
            }

            fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
                unimplemented!()
            }
        }

        let bxdf = TestBxDF{};
        assert!(bxdf.has_flag(REFLECTION));
        assert!(bxdf.has_flag(DIFFUSE));
        assert_eq!(bxdf.has_flag(TRANSMISSION), false);
        assert_eq!(bxdf.has_flag(GLOSSY), false);
        assert_eq!(bxdf.has_flag(SPECULAR), false);
    }

    #[test]
    fn test_bsdf_world_loacl() {
        let bsdf = BSDF::new(
            1.,
            &Interaction {
                p: Point3f::new(1., 1., 1.),
                n: Normal3f::new(-1., 0., 0.),
                dpdu: Vector3f::new(0., -1., 0.),
                ..Default::default()
            },
        );

        let v = Vector3f::new(1., 2., 3.);

        let v1 = bsdf.local_to_world(v);
        assert_eq!(v1, Vector3f::new(-3., -1., 2.));

        let v2 = bsdf.world_to_local(v1);
        for i in 0..3 {
            assert!((v2[i] - v[i]).abs() < EPSILON);
        }
    }
}
