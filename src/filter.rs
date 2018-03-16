use vector::Point2f;

pub trait Filter {
    fn radius(&self) -> f32;

    /// A point relative to the center of Filter.
    fn evaluate(&self, p: Point2f) -> f32;
}

pub struct GaussianFilter {
    radius: f32,
    alpha: f32,
    exp: f32,
}

impl GaussianFilter {
    pub fn new(radius: f32, alpha: f32) -> GaussianFilter {
        GaussianFilter {
            radius,
            alpha,
            exp: (-alpha * radius.powi(2)).exp(),
        }
    }

    fn gaussian(&self, d: f32) -> f32 {
        ((-self.alpha * d * d).exp() - self.exp).max(0.)
    }
}

impl Filter for GaussianFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: Point2f) -> f32 {
        self.gaussian(p.x) * self.gaussian(p.y)
    }
}
