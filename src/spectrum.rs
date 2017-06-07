use std::ops;

use common::Float;
use common::clamp;

pub struct RGB {
    pub r: Float,
    pub g: Float,
    pub b: Float,
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
    x: Float,
    y: Float,
    z: Float,
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

pub type Spectrum = RGBSpectrum;

#[derive(Clone, Copy)]
pub struct RGBSpectrum {
    r: Float,
    g: Float,
    b: Float,
}
impl_vector3f_new_and_ops!(RGBSpectrum, r, g, b);
impl_vector3f_add!(RGBSpectrum, RGBSpectrum, RGBSpectrum, r, g, b);

impl From<RGB> for RGBSpectrum {
    fn from(RGB { r, g, b }: RGB) -> RGBSpectrum {
        RGBSpectrum::new(r, g, b)
    }
}

impl From<XYZ> for RGBSpectrum {
    fn from(xyz: XYZ) -> RGBSpectrum {
        RGBSpectrum::from(RGB::from(xyz))
    }
}

impl From<RGBSpectrum> for RGB {
    fn from(RGBSpectrum { r, g, b }: RGBSpectrum) -> RGB {
        RGB::new(r, g, b)
    }
}

impl From<RGBSpectrum> for XYZ {
    fn from(rgbs: RGBSpectrum) -> XYZ {
        XYZ::from(RGB::from(rgbs))
    }
}

impl RGBSpectrum {
    fn is_black(&self) -> bool {
        self.r == 0. && self.g == 0. && self.b == 0.
    }

    fn sqrt(&self) -> RGBSpectrum {
        RGBSpectrum::new(
            self.r.sqrt(),
            self.g.sqrt(),
            self.b.sqrt(),
        )
    }

    fn lerp(&self, s: &RGBSpectrum, t: Float) -> RGBSpectrum {
        *self * (1. - t) + *s * t
    }

    fn clamp(&self, low: Float, high: Float) -> RGBSpectrum {
        RGBSpectrum::new(
            clamp(self.r, low, high),
            clamp(self.g, low, high),
            clamp(self.b, low, high),
        )
    }
}
