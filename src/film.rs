use std::io::Write;

use bbox::{BBox2u, BBox2f};
use common::clamp;
use spectrum::Spectrum;
use vector::{Vector2u, Vector2f, Point2u, Point2f};

pub struct Film {
    pub resolution: Point2u,
    pixels: Box<[Pixel]>,
}

impl Film {
    pub fn new(resolution: Point2u) -> Film {
        let area = (resolution.x * resolution.y) as usize;
        let pixels = {
            let mut pixels = Vec::with_capacity(area);
            for _ in 0..area {
                pixels.push(Pixel::default());
            }
            pixels.into_boxed_slice()
        };

        return Film { resolution, pixels };
    }

    pub fn get_film_tile(&self, bbox: BBox2u) -> FilmTile {
        let bbox = BBox2f::from(bbox);
        let half_pixel = Vector2f::new(0.5, 0.5);
        let min = (bbox.min - half_pixel).ceil();
        let max = (bbox.max + half_pixel).floor();
        let bbox = BBox2u::from(BBox2f::new(min, max));
        return FilmTile::new(bbox);
    }

    fn pixel_offset(&self, Point2u { x, y }: Point2u) -> usize {
        let width = self.resolution.x;
        return (width * y + x) as usize;
    }

    pub fn merge_film_tile(&mut self, tile: &FilmTile) {
        for pixel in tile.bbox.iter() {
            let p = &tile.pixels[tile.pixel_offset(pixel)];
            self.pixels[self.pixel_offset(pixel)].merge(p);
        }
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

pub struct FilmTile {
    pub bbox: BBox2u,
    pixels: Box<[Pixel]>,
}

impl FilmTile {
    fn new(bbox: BBox2u) -> FilmTile {
        let area = bbox.area() as usize;
        let pixels = {
            let mut pixels = Vec::with_capacity(area);
            for _ in 0..area {
                pixels.push(Pixel::default());
            }
            pixels.into_boxed_slice()
        };

        return FilmTile { bbox, pixels };
    }

    fn pixel_offset(&self, p_film: Point2u) -> usize {
        let Vector2u { x, y } = p_film - self.bbox.min;
        let width = self.bbox.max.x - self.bbox.min.x;
        return (width * y + x) as usize;
    }

    pub fn add_sample(&mut self, p_film: Point2f, sample: Spectrum) {
        let mut p_film = Point2u::from(p_film.floor());
        for i in 0..2 {
            if p_film[i] >= self.bbox.max[i] {
                p_film[i] -= 1;
            }
        }
        let pixel_offset = self.pixel_offset(p_film);
        self.pixels[pixel_offset].add_sample(sample);
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

    fn merge(&mut self, pixel: &Pixel) {
        if self.n_samples == 0 {
            self.n_samples = pixel.n_samples;
            self.color = pixel.color;
        } else {
            self.color = (self.color * self.n_samples as f32) + (pixel.color * pixel.n_samples as f32)
                       / (self.n_samples + pixel.n_samples) as f32;
            self.n_samples += pixel.n_samples;
        }
    }
}

fn gamma_correct(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}
