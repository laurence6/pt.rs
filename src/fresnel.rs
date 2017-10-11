use std::mem::swap;

use common::clamp;
use spectrum::Spectrum;

pub trait Fresnel {
    fn evaluate(&self, cos_i: f32) -> Spectrum;
}

pub struct FresnelDielectric {
    eta_i: f32,
    eta_t: f32,
}

impl FresnelDielectric {
    pub fn new(eta_i: f32, eta_t: f32) -> FresnelDielectric {
        FresnelDielectric { eta_i, eta_t }
    }
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        Spectrum::from(fr_dielectric(cos_i, self.eta_i, self.eta_t))
    }
}

pub struct FresnelConductor {
    eta_i: Spectrum,
    eta_t: Spectrum,
    k: Spectrum,
}

impl FresnelConductor {
    pub fn new(eta_i: Spectrum, eta_t: Spectrum, k: Spectrum) -> FresnelConductor {
        FresnelConductor { eta_i, eta_t, k }
    }
}

impl Fresnel for FresnelConductor {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        fr_conductor(cos_i.abs(), self.eta_i, self.eta_t, self.k)
    }
}

pub struct FresnelNoOp {}

impl FresnelNoOp {
    pub fn new() -> FresnelNoOp {
        FresnelNoOp {}
    }
}

impl Fresnel for FresnelNoOp {
    fn evaluate(&self, cos_i: f32) -> Spectrum {
        Spectrum::from(1.)
    }
}

fn fr_dielectric(mut cos_theta_i: f32, mut eta_i: f32, mut eta_t: f32) -> f32 {
    cos_theta_i = clamp(cos_theta_i, -1., 1.);

    let entering = cos_theta_i > 0.;
    if !entering {
        swap(&mut eta_i, &mut eta_t);
        cos_theta_i *= -1.;
    }

    let sin_theta_i = (1. - cos_theta_i * cos_theta_i).sqrt();

    let sin_theta_t = eta_i / eta_t * sin_theta_i;
    if sin_theta_t >= 1. { // total internal reflection
        return 1.;
    }
    let cos_theta_t = (1. - sin_theta_t * sin_theta_t).sqrt();

    let r_parl = (eta_t * cos_theta_i - eta_i * cos_theta_t)
               / (eta_t * cos_theta_i + eta_i * cos_theta_t);
    let r_perp = (eta_i * cos_theta_i - eta_t * cos_theta_t)
               / (eta_i * cos_theta_i + eta_t * cos_theta_t);
    return (r_parl * r_parl + r_perp * r_perp) / 2.;
}

fn fr_conductor(mut cos_theta_i: f32, eta_i: Spectrum, eta_t: Spectrum, k: Spectrum) -> Spectrum {
    cos_theta_i = clamp(cos_theta_i, -1., 1.);
    let eta = eta_t / eta_i;
    let eta_k = k / eta_i;

    let cos_theta_i_2 = cos_theta_i * cos_theta_i;
    let sin_theta_i_2 = 1. - cos_theta_i_2;
    let eta_2 = eta * eta;
    let eta_k_2 = eta_k * eta_k;

    let t0 = eta_2 - eta_k_2 - sin_theta_i_2;
    let a_2_plus_b_2 = (t0 * t0 + eta_2 * eta_k_2 * 4.).sqrt();
    let t1 = a_2_plus_b_2 + cos_theta_i_2;
    let a = ((a_2_plus_b_2 + t0) * 0.5).sqrt();
    let t2 = a * cos_theta_i * 2.;
    let r_perp = (t1 - t2) / (t1 + t2);

    let t3 = a_2_plus_b_2 * cos_theta_i_2 + sin_theta_i_2 * sin_theta_i_2;
    let t4 = t2 * sin_theta_i_2;
    let r_parl = r_perp * (t3 - t4) / (t3 + t4);

    return (r_parl + r_perp) * 0.5;
}
