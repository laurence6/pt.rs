use std::ops;

use common::Float;
use common::clamp;

pub type Spectrum = RGBSpectrum;

struct RGB {
    r: Float,
    g: Float,
    b: Float,
}
impl_vector3f!(RGB, r, g, b);

impl From<XYZ> for RGB {
    fn from(xyz: XYZ) -> RGB {
        let r =  3.240479 * xyz.x + -1.537150 * xyz.y + -0.498535 * xyz.z;
        let g = -0.969256 * xyz.x +  1.875991 * xyz.y +  0.041556 * xyz.z;
        let b =  0.055648 * xyz.x + -0.204043 * xyz.y +  1.057311 * xyz.z;
        return RGB::new(r, g, b);
    }
}

struct XYZ {
    x: Float,
    y: Float,
    z: Float,
}
impl_vector3f!(XYZ, x, y, z);

impl From<RGB> for XYZ {
    fn from(rgb: RGB) -> XYZ {
        let x = 0.412453 * rgb.r + 0.357580 * rgb.g + 0.180423 * rgb.b;
        let y = 0.212671 * rgb.r + 0.715160 * rgb.g + 0.072169 * rgb.b;
        let z = 0.019334 * rgb.r + 0.119193 * rgb.g + 0.950227 * rgb.b;
        return XYZ::new(x, y, z);
    }
}


#[derive(Clone, Copy)]
pub struct RGBSpectrum {
    r: Float,
    g: Float,
    b: Float,
}
impl_vector3f!(RGBSpectrum, r, g, b);
impl_vector3f_add!(RGBSpectrum, RGBSpectrum, RGBSpectrum, r, g, b);

impl From<RGB> for RGBSpectrum {
    fn from(rgb: RGB) -> RGBSpectrum {
        RGBSpectrum::new(rgb.r, rgb.g, rgb.b)
    }
}

impl From<XYZ> for RGBSpectrum {
    fn from(xyz: XYZ) -> RGBSpectrum {
        RGBSpectrum::from(RGB::from(xyz))
    }
}

impl From<RGBSpectrum> for RGB {
    fn from(rgbs: RGBSpectrum) -> RGB {
        RGB::new(rgbs.r, rgbs.g, rgbs.b)
    }
}

impl From<RGBSpectrum> for XYZ {
    fn from(rgbs: RGBSpectrum) -> XYZ {
        XYZ::from(RGB::from(rgbs))
    }
}

impl RGBSpectrum {
    fn is_black(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    fn sqrt(&self) -> RGBSpectrum {
        RGBSpectrum::new(
            self.r.sqrt(),
            self.g.sqrt(),
            self.b.sqrt(),
        )
    }

    fn lerp(&self, s: &RGBSpectrum, t: Float) -> RGBSpectrum {
        *self * (1.0 - t) + *s * t
    }

    fn clamp(&self, low: Float, high: Float) -> RGBSpectrum {
        RGBSpectrum::new(
            clamp(self.r, low, high),
            clamp(self.g, low, high),
            clamp(self.b, low, high),
        )
    }
}
