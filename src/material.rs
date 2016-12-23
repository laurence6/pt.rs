use common::Float;
use std::cmp::{min, max};

#[derive(Debug)]
pub struct Color {
    pub R: Float,
    pub G: Float,
    pub B: Float,
}

pub fn NewColor(r: Float, g: Float, b: Float) -> Color {
    return Color { R: r, G: g, B: b };
}

const GAMMA_CORRECTION: Float = 2.2;
pub fn HexColor(x: u32) -> Color {
    let r = (((x >>16) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
    let g = (((x >> 8) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
    let b = (((x >> 0) & 0xff) as Float / 255.0).powf(GAMMA_CORRECTION);
    return Color { R: r, G: g, B: b };
}

impl Color {
    pub fn RGB(&self) -> (u32, u32, u32) {
        return (toInt(self.R), toInt(self.G), toInt(self.B));
    }
}

fn clamp(x: Float) -> Float {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}

fn toInt(x: Float) -> u32 {
    return max(0, min(255, (clamp(x) * 255.0) as u32));
}
