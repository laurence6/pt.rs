use common::clamp;

pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl_vector3f_new_and_ops!(RGB, r, g, b);
impl_vector3f_add!(RGB, RGB, RGB, r, g, b);
impl_vector3f_sub!(RGB, RGB, RGB, r, g, b);

impl From<XYZ> for RGB {
    fn from(XYZ { x, y, z }: XYZ) -> RGB {
        let r =  3.240479 * x + -1.537150 * y + -0.498535 * z;
        let g = -0.969256 * x +  1.875991 * y +  0.041556 * z;
        let b =  0.055648 * x + -0.204043 * y +  1.057311 * z;
        return RGB::new(r, g, b);
    }
}

#[derive(Default, Clone, Copy)]
pub struct XYZ {
    x: f32,
    y: f32,
    z: f32,
}
impl_vector3f_new_and_ops!(XYZ, x, y, z);
impl_vector3f_add!(XYZ, XYZ, XYZ, x, y, z);
impl_vector3f_sub!(XYZ, XYZ, XYZ, x, y, z);

impl From<RGB> for XYZ {
    fn from(RGB { r, g, b }: RGB) -> XYZ {
        let x = 0.412453 * r + 0.357580 * g + 0.180423 * b;
        let y = 0.212671 * r + 0.715160 * g + 0.072169 * b;
        let z = 0.019334 * r + 0.119193 * g + 0.950227 * b;
        return XYZ::new(x, y, z);
    }
}

#[derive(Default, Clone, Copy)]
pub struct Spectrum {
    r: f32,
    g: f32,
    b: f32,
}
impl_vector3f_new_and_ops!(Spectrum, r, g, b);
impl_vector3f_add!(Spectrum, Spectrum, Spectrum, r, g, b);
impl_vector3f_mul!(Spectrum, Spectrum, Spectrum, r, g, b);

impl From<RGB> for Spectrum {
    fn from(RGB { r, g, b }: RGB) -> Spectrum {
        Spectrum::new(r, g, b)
    }
}

impl From<XYZ> for Spectrum {
    fn from(xyz: XYZ) -> Spectrum {
        Spectrum::from(RGB::from(xyz))
    }
}

impl From<Spectrum> for RGB {
    fn from(Spectrum { r, g, b }: Spectrum) -> RGB {
        RGB::new(r, g, b)
    }
}

impl From<Spectrum> for XYZ {
    fn from(rgbs: Spectrum) -> XYZ {
        XYZ::from(RGB::from(rgbs))
    }
}

impl Spectrum {
    pub fn is_black(&self) -> bool {
        self.r == 0. && self.g == 0. && self.b == 0.
    }

    fn sqrt(&self) -> Spectrum {
        Spectrum::new(
            self.r.sqrt(),
            self.g.sqrt(),
            self.b.sqrt(),
        )
    }

    fn lerp(&self, s: &Spectrum, t: f32) -> Spectrum {
        *self * (1. - t) + *s * t
    }

    fn clamp(&self, low: f32, high: f32) -> Spectrum {
        Spectrum::new(
            clamp(self.r, low, high),
            clamp(self.g, low, high),
            clamp(self.b, low, high),
        )
    }
}
