use common::clamp;
use interaction::Interaction;
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

    /// Return the direction of incident ray, value of distribution function, pdf.
    fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32);

    /// Return pdf for the given pair of direction.
    fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32;
}

pub struct BSDF {
    bxdfs: Vec<Box<BxDF>>,
    gn: Vector3f, // geometric normal
    sn: Vector3f, // shading normal
    s: Vector3f,
    t: Vector3f,
}

impl BSDF {
    pub fn new(i: &Interaction) -> BSDF {
        let gn = Vector3f::from(i.n);
        let sn = Vector3f::from(i.sn);
        let s = i.dpdu.normalize();
        let t = sn.cross(s);
        return BSDF {
            bxdfs: Vec::new(),
            gn,
            sn,
            s,
            t,
        };
    }

    /// Add a BxDF component.
    pub fn add<T>(&mut self, bxdf: T) where T: 'static + BxDF {
        self.bxdfs.push(Box::new(bxdf));
    }

    fn world_to_local(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            v.dot(self.s),
            v.dot(self.t),
            v.dot(self.sn),
        )
    }

    fn local_to_world(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.s.x * v.x + self.t.x * v.y + self.sn.x * v.z,
            self.s.y * v.x + self.t.y * v.y + self.sn.y * v.z,
            self.s.z * v.x + self.t.z * v.y + self.sn.z * v.z,
        )
    }

    pub fn f(&self, wo_w: Vector3f, wi_w: Vector3f) -> Spectrum {
        let wo = self.world_to_local(wo_w);
        if wo.z == 0. {
            return Default::default();
        }
        let wi = self.world_to_local(wi_w);

        let reflect = wi_w.dot(self.gn) * wo_w.dot(self.gn) > 0.;

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

        if !bxdf.has_flag(SPECULAR) && n_bxdfs > 1 {
            for i in 0..n_bxdfs {
                if i != n_bxdf {
                    pdf += self.bxdfs[i].pdf(wo, wi);
                }
            }

            let reflect = wi_w.dot(self.gn) * wo_w.dot(self.gn) > 0.;
            f = Spectrum::default();
            for bxdf in self.bxdfs.iter() {
                if (reflect && bxdf.has_flag(REFLECTION))
                    || (!reflect && bxdf.has_flag(TRANSMISSION)) {
                    f += bxdf.f(wo, wi);
                }
            }
        }
        if n_bxdfs > 1 {
            pdf /= n_bxdfs as f32;
        }

        return (wi_w, f, pdf, bxdf.flag());
    }

    pub fn pdf(&self, wo_w: Vector3f, wi_w: Vector3f) -> f32 {
        let n_bxdfs = self.bxdfs.len();
        if n_bxdfs == 0 {
            return 0.;
        }

        let wo = self.world_to_local(wo_w);
        let wi = self.world_to_local(wi_w);
        if wo.z == 0. {
            return 0.;
        }

        let mut pdf = 0.;
        for bxdf in self.bxdfs.iter() {
            pdf += bxdf.pdf(wo, wi);
        }

        return pdf / n_bxdfs as f32;
    }
}

pub fn cos_theta(w: Vector3f) -> f32 {
    w.z
}

pub fn cos_2_theta(w: Vector3f) -> f32 {
    w.z * w.z
}

pub fn abs_cos_theta(w: Vector3f) -> f32 {
    w.z.abs()
}

pub fn sin_theta(w: Vector3f) -> f32 {
    sin_2_theta(w).sqrt()
}

pub fn sin_2_theta(w: Vector3f) -> f32 {
    (1. - cos_2_theta(w)).max(0.)
}

pub fn tan_theta(w: Vector3f) -> f32 {
    sin_theta(w) / cos_theta(w)
}

pub fn tan_2_theta(w: Vector3f) -> f32 {
    sin_2_theta(w) / cos_2_theta(w)
}

pub fn cos_phi(w: Vector3f) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        return 1.
    } else {
        return clamp(w.x / sin_theta, -1., 1.);
    }
}

pub fn sin_phi(w: Vector3f) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        return 0.
    } else {
        return clamp(w.y / sin_theta, -1., 1.);
    }
}

pub fn cos_2_phi(w: Vector3f) -> f32 {
    cos_phi(w).powi(2)
}

pub fn sin_2_phi(w: Vector3f) -> f32 {
    sin_phi(w).powi(2)
}

pub fn same_hemisphere(w1: Vector3f, w2: Vector3f) -> bool {
    w1.z * w2.z > 0.
}

pub fn reflect(w: Vector3f, n: Vector3f) -> Vector3f {
    -w + n * w.dot(n) * 2.
}

pub fn refract(wi: Vector3f, n: Vector3f, eta: f32) -> Option<Vector3f> {
    let cos_theta_i = wi.dot(n);

    let sin_2_theta_i = (1. - cos_theta_i * cos_theta_i).max(0.);
    let sin_2_theta_t = eta * eta * sin_2_theta_i;

    if sin_2_theta_t >= 1. { // total internal reflection
        return None;
    }

    let cos_theta_t = (1. - sin_2_theta_t).sqrt();
    let wt = -wi * eta + n * (cos_theta_i * eta - cos_theta_t);
    return Some(wt);
}

#[cfg(test)]
mod test {
    use common::EPSILON;
    use interaction::Interaction;
    use reflection::{BxDF, BSDF, BxDFFlag, REFLECTION, TRANSMISSION, DIFFUSE, GLOSSY, SPECULAR};
    use spectrum::Spectrum;
    use vector::{Vector3f, Normal3f, Point3f, Point2f};

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

            fn sample_f(&self, wo: Vector3f, sample: Point2f) -> (Vector3f, Spectrum, f32) {
                unimplemented!()
            }

            fn pdf(&self, wo: Vector3f, wi: Vector3f) -> f32 {
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
            &Interaction {
                p: Point3f::new(1., 1., 1.),
                n: Normal3f::new(-1., 0., 0.),
                sn: Normal3f::new(-1., 0., 0.),
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
