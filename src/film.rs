use std::io::Write;

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

    pub fn add_sample(&mut self, p_film: Point2f, sample: Spectrum) {
        self.pixels[self.pixel_offset(p_film)].add_sample(sample);
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

    /// Write an image file in plain ppm format.
    pub fn write_image_ppm<T>(&self, file: &mut T) where T: Write {
        let header = format!("P3\n{} {}\n255\n", self.resolution.x, self.resolution.y);
        file.write_all(header.as_bytes()).unwrap();

        for p in self.pixels.iter() {
            let Spectrum { r, g, b } = (p.color * 255.).clamp(0., 255.);
            file.write_all(format!(
                "{} {} {}\n",
                r.round() as u32,
                g.round() as u32,
                b.round() as u32,
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
