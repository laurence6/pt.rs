use std::io::Write;

use common::clamp;
use spectrum::Spectrum;
use vector::{Point2u, Point2f};

pub struct Film {
    pub resolution: Point2u,
    pixels: Box<[Pixel]>,
}

impl Film {
    pub fn new(resolution: Point2u) -> Film {
        let area = (resolution.x * resolution.y) as usize;
        let pixels = {
            let mut pixels = Vec::with_capacity(area);
            for i in 0..area {
                pixels.push(Pixel::default());
            }
            pixels.into_boxed_slice()
        };

        return Film { resolution, pixels };
    }

    fn pixel_offset(&self, Point2f { x, y }: Point2f) -> usize {
        let (width, height) = (self.resolution.x as usize, self.resolution.y as usize);
        let (mut x, mut y) = (x.floor() as usize, y.floor() as usize);
        if x >= width {
            x = width - 1;
        }
        if y >= height {
            y = height - 1;
        }
        return y * width + x;
    }

    pub fn add_sample(&mut self, p_film: Point2f, sample: Spectrum) {
        self.pixels[self.pixel_offset(p_film)].add_sample(sample);
    }

    /// Write an image file in plain ppm format.
    pub fn write_image_ppm<T>(&self, file: &mut T) where T: Write {
        let header = format!("P3\n{} {}\n255\n", self.resolution.x, self.resolution.y);
        file.write_all(header.as_bytes()).unwrap();

        for p in self.pixels.iter() {
            let mut color = p.color;
            for i in 0..3 {
                color[i] = clamp(gamma_correct(color[i]) * 255. + 0.5, 0., 255.).round();
            }
            let Spectrum { r, g, b } = color;
            file.write_all(format!(
                "{} {} {}\n",
                r as u32,
                g as u32,
                b as u32,
            ).as_bytes()).unwrap();
        }
    }
}

#[derive(Default)]
struct Pixel {
    n_samples: u32,
    color: Spectrum,
}

impl Pixel {
    fn add_sample(&mut self, sample: Spectrum) {
        self.n_samples += 1;
        self.color += (sample - self.color) / self.n_samples as f32;
    }
}

fn gamma_correct(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}
