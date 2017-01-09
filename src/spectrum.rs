use std::ops;

use common::Float;
use common::Clamp;

pub type Spectrum = RGBSpectrum;

type RGB = [Float; 3];
type XYZ = [Float; 3];

#[derive(Clone, Copy)]
pub struct RGBSpectrum (Float, Float, Float);

impl RGBSpectrum {
    pub fn FromRGB(rgb: RGB) -> RGBSpectrum {
        return RGBSpectrum(rgb[0], rgb[1], rgb[2]);
    }

    pub fn FromXYZ(xyz: XYZ) -> RGBSpectrum {
        return RGBSpectrum::FromRGB(XYZToRGB(xyz));
    }

    pub fn ToRGB(&self) -> RGB {
        return [self.0, self.1, self.2];
    }

    pub fn ToXYZ(&self) -> XYZ {
        return RGBToXYZ(self.ToRGB());
    }

    pub fn ToRGBSpectrum(&self) -> RGBSpectrum {
        return *self;
    }

    pub fn IsBlack(&self) -> bool {
        self.0 == 0.0 && self.1 == 0.0 && self.2 == 0.0
    }

    pub fn Sqrt(&self) -> RGBSpectrum {
        return RGBSpectrum(self.0.sqrt(), self.1.sqrt(), self.2.sqrt());
    }

    pub fn Lerp(&self, s: &RGBSpectrum, t: Float) -> RGBSpectrum {
        return *self * (1.0 - t) + *s * t;
    }

    pub fn Clamp(&self, low: Float, high: Float) -> RGBSpectrum {
        return RGBSpectrum(
            Clamp(self.0, low, high),
            Clamp(self.1, low, high),
            Clamp(self.2, low, high),
        );
    }
}

impl ops::Neg for RGBSpectrum {
    type Output = RGBSpectrum;
    fn neg(self) -> RGBSpectrum {
        return RGBSpectrum(-self.0, -self.1, -self.2);
    }
}

impl ops::Add<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn add(self, s: RGBSpectrum) -> RGBSpectrum {
        return RGBSpectrum(self.0 + s.0, self.1 + s.1, self.2 + s.2);
    }
}

impl ops::Sub<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn sub(self, s: RGBSpectrum) -> RGBSpectrum {
        return RGBSpectrum(self.0 - s.0, self.1 - s.1, self.2 - s.2);
    }
}

impl ops::Add<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn add(self, a: Float) -> RGBSpectrum {
        return RGBSpectrum(self.0 + a, self.1 + a, self.2 + a);
    }
}

impl ops::Sub<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn sub(self, a: Float) -> RGBSpectrum {
        return RGBSpectrum(self.0 - a, self.1 - a, self.2 - a);
    }
}

impl ops::Mul<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn mul(self, a: Float) -> RGBSpectrum {
        return RGBSpectrum(self.0 * a, self.1 * a, self.2 * a);
    }
}

impl ops::Div<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn div(self, a: Float) -> RGBSpectrum {
        return RGBSpectrum(self.0 / a, self.1 / a, self.2 / a);
    }
}

fn XYZToRGB(xyz: XYZ) -> RGB {
    let r =  3.240479 * xyz[0] + -1.537150 * xyz[1] + -0.498535 * xyz[2];
    let g = -0.969256 * xyz[0] +  1.875991 * xyz[1] +  0.041556 * xyz[2];
    let b =  0.055648 * xyz[0] + -0.204043 * xyz[1] +  1.057311 * xyz[2];
    return [r, g, b];
}

fn RGBToXYZ(rgb: RGB) -> XYZ {
    let x = 0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2];
    let y = 0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2];
    let z = 0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2];
    return [x, y, z];
}
