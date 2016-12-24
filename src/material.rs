use common::Float;
use common::clamp;
use std::cmp::{min, max};

const GAMMA_CORRECTION: Float = 2.2;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub R: Float,
    pub G: Float,
    pub B: Float,
}

impl Color {
    pub fn New(r: Float, g: Float, b: Float) -> Color {
        return Color { R: r, G: g, B: b };
    }

    pub fn Hex(x: u32) -> Color {
        let r = (((x >>16) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
        let g = (((x >> 8) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
        let b = (((x >> 0) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
        return Color { R: r, G: g, B: b };
    }

    pub fn RGB(&self) -> (u32, u32, u32) {
        return (toInt(self.R), toInt(self.G), toInt(self.B));
    }
}

fn toInt(x: Float) -> u32 {
    return max(0, min(255, (clamp(x) * 255.0) as u32));
}
