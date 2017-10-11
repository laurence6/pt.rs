use common::clamp;

#[derive(Default, Clone, Copy, Debug)]
pub struct Spectrum {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl_vector3f_new_and_ops!(Spectrum, r, g, b);
impl_vector3f_add!(Spectrum, Spectrum, Spectrum, r, g, b);
impl_vector3f_sub!(Spectrum, Spectrum, Spectrum, r, g, b);
impl_vector3f_mul!(Spectrum, Spectrum, Spectrum, r, g, b);
impl_vector3f_div!(Spectrum, Spectrum, Spectrum, r, g, b);
impl_vector3f_index!(Spectrum, r, g, b);

impl Spectrum {
    pub fn is_black(&self) -> bool {
        self.r == 0. && self.g == 0. && self.b == 0.
    }

    pub fn y(&self) -> f32 {
          0.212671 * self.r
        + 0.715160 * self.g
        + 0.072169 * self.b
    }

    pub fn sqrt(&self) -> Spectrum {
        Spectrum::new(
            self.r.sqrt(),
            self.g.sqrt(),
            self.b.sqrt(),
        )
    }

    fn lerp(&self, s: &Spectrum, t: f32) -> Spectrum {
        *self * (1. - t) + *s * t
    }

    pub fn clamp(&self, low: f32, high: f32) -> Spectrum {
        Spectrum::new(
            clamp(self.r, low, high),
            clamp(self.g, low, high),
            clamp(self.b, low, high),
        )
    }
}

impl From<f32> for Spectrum {
    fn from(v: f32) -> Spectrum {
        Spectrum::new(v, v, v)
    }
}
