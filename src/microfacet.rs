use common::PI;
use reflection::{cos_2_theta, abs_cos_theta, tan_theta, tan_2_theta, cos_2_phi, sin_2_phi, same_hemisphere};
use vector::{Vector3f, Point2f};

/// GGX(Trowbridge–Reitz) Microfacet Distribution.
#[derive(Clone, Copy)]
pub struct MicrofacetDistribution {
    alpha_x: f32,
    alpha_y: f32,
}

impl MicrofacetDistribution {
    pub fn new(alpha_x: f32, alpha_y: f32) -> MicrofacetDistribution {
        MicrofacetDistribution { alpha_x, alpha_y }
    }

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

    pub fn g1(&self, w: Vector3f) -> f32 {
        1. / (1. + self.lambda(w))
    }

    pub fn g(&self, wo: Vector3f, wi: Vector3f) -> f32 {
        1. / (1. + self.lambda(wo) + self.lambda(wi))
    }

    pub fn sample_wh(&self, wo: Vector3f, sample: Point2f) -> Vector3f {
        let cos_theta;
        let mut phi = PI * 2. * sample[1];
        if self.alpha_x == self.alpha_y {
            let tan_theta_2 = self.alpha_x * self.alpha_x * sample[0] / (1. - sample[0]);
            cos_theta = 1. / (1. + tan_theta_2).sqrt();
        } else {
            phi = (self.alpha_y / self.alpha_x * (2. * PI * sample[1] + 0.5 * PI).tan()).atan();
            if sample[1] > 0.5 {
                phi += PI;
            }
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            let alpha_x_2 = self.alpha_x * self.alpha_x;
            let alpha_y_2 = self.alpha_y * self.alpha_y;
            let alpha_2 = 1. / (cos_phi * cos_phi / alpha_x_2 + sin_phi * sin_phi / alpha_y_2);
            let tan_theta_2 = alpha_2 * sample[0] / (1. - sample[0]);
            cos_theta = 1. / (1. + tan_theta_2).sqrt();
        }

        let sin_theta = (1. - cos_theta * cos_theta).max(0.).sqrt();

        let mut wh = Vector3f::from_spherical_direction(sin_theta, cos_theta, phi);
        if !same_hemisphere(wo, wh) {
            wh *= -1.;
        }

        return wh;
    }

    pub fn pdf(&self, wo: Vector3f, wh: Vector3f) -> f32 {
        self.d(wh) / abs_cos_theta(wh)
    }
}
