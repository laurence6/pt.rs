use common::PI;
use reflection::{cos_2_theta, tan_theta, tan_2_theta, cos_2_phi, sin_2_phi};
use vector::Vector3f;

/// GGX(Trowbridge–Reitz) Microfacet Distribution.
pub struct MicrofacetDistribution {
    alpha_x: f32,
    alpha_y: f32,
}

impl MicrofacetDistribution {
    pub fn d(&self, wh: Vector3f) -> f32 {
        let tan_2_theta = tan_2_theta(wh);
        if tan_2_theta.is_infinite() {
            return 0.;
        }
        let cos_4_theta = cos_2_theta(wh).powi(2);
        let e = (cos_2_phi(wh) / self.alpha_x.powi(2) + sin_2_phi(wh) / self.alpha_y.powi(2))
              * tan_2_theta;
        return 1.
            / (PI * self.alpha_x * self.alpha_y * cos_4_theta * (1. + e).powi(2));
    }

    /// Invisible masked microfacet area per visible microfacet area.
    fn lambda(&self, w: Vector3f) -> f32 {
        let abs_tan_theta = tan_theta(w).abs();
        if abs_tan_theta.is_infinite() {
            return 0.;
        }
        let alpha = (cos_2_phi(w) * self.alpha_x.powi(2) + sin_2_phi(w) * self.alpha_y.powi(2)).sqrt();
        let alpha_2_tan_2_theta = (alpha * abs_tan_theta).powi(2);
        return (-1. + (1. + alpha_2_tan_2_theta).sqrt()) * 0.5;
    }

    pub fn g(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        1. / (1. + self.lambda(wo) + self.lambda(wi))
    }
}
