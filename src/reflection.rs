use common::Float;
use common::clamp;
use vector::{Vector3f, Point2f};
use spectrum::Spectrum;

type BxDFType = u8;
pub const REFLECTION:   BxDFType = 1 << 0;
pub const TRANSMISSION: BxDFType = 1 << 1;
pub const DIFFUSE:      BxDFType = 1 << 2;
pub const GLOSSY:       BxDFType = 1 << 3;
pub const SPECULAR:     BxDFType = 1 << 4;
pub const ALL:          BxDFType = REFLECTION | TRANSMISSION | DIFFUSE | GLOSSY | SPECULAR;

pub struct BxDF {
    bxdf_type: BxDFType,
}

impl BxDF {
    pub fn match_type(&self, t: BxDFType) -> bool {
        debug_assert!(self.bxdf_type <= ALL);

        (self.bxdf_type & t) == self.bxdf_type
    }

    pub fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        unimplemented!()
    }

    pub fn sample_f(&self, wo: Vector3f, wi: &mut Vector3f, sample: Point2f, pdf: &mut Float, sampled_type: &mut BxDFType) -> Spectrum {
        unimplemented!()
    }

    pub fn rho(&self) {}
}


fn cos_theta(Vector3f { z, .. }: Vector3f) -> Float {
    z
}

fn cos2_theta(Vector3f { z, .. }: Vector3f) -> Float {
    z * z
}

fn abscos_theta(Vector3f { z, .. }: Vector3f) -> Float {
    z.abs()
}

fn sin_theta(w: Vector3f) -> Float {
    sin2_theta(w).sqrt()
}

fn sin2_theta(w: Vector3f) -> Float {
    (1. - cos2_theta(w)).max(0.)
}

fn tan_theta(w: Vector3f) -> Float {
    sin_theta(w) / cos_theta(w)
}

fn tan2_theta(w: Vector3f) -> Float {
    sin2_theta(w) / cos2_theta(w)
}

fn cos_phi(w: Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        return 1.;
    } else {
        return clamp(w.x / sin_theta, -1., 1.);
    }
}

fn cos2_phi(w: Vector3f) -> Float {
    cos_phi(w) * cos_phi(w)
}

fn sin_phi(w: Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        return 0.;
    } else {
        return clamp(w.y / sin_theta, -1., 1.);
    }
}

fn sin2_phi(w: Vector3f) -> Float {
    sin_phi(w) * sin_phi(w)
}

fn cos_dphi(wa: Vector3f, wb: Vector3f) -> Float {
    clamp(
           (wa.x * wb.x + wa.y * wb.y)
        / ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y).sqrt()),
        -1., 1.,
    )
}
