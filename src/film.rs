use std::cmp;
use std::io::Write;
use std::sync::{Arc, Mutex};

use bbox::{BBox2u, BBox2f};
use common::clamp;
use filter::Filter;
use spectrum::Spectrum;
use vector::{Vector2f, Point2u, Point2f};

const FILTER_TABLE_WIDTH: usize = 16;
type FilterTable = [[f32; FILTER_TABLE_WIDTH]; FILTER_TABLE_WIDTH];

const TILE_SIZE: u32 = 16;

const HALF_PIXEL: Vector2f = Vector2f { x: 0.5, y: 0.5 };

pub struct Film {
    pub resolution: Point2u,
    filter_radius: f32,
    filter_table: FilterTable,
    pixels: Mutex<Box<[Pixel]>>,
}

impl Film {
    pub fn new<T>(resolution: Point2u, filter: T) -> Film where T: Filter {
        let mut filter_table = [[0.; FILTER_TABLE_WIDTH]; FILTER_TABLE_WIDTH];
        let filter_radius = filter.radius();
        for y in 0..FILTER_TABLE_WIDTH {
            for x in 0..FILTER_TABLE_WIDTH {
                let p = Point2f::new(
                    (x as f32 + 0.5) * filter_radius / FILTER_TABLE_WIDTH as f32,
                    (y as f32 + 0.5) * filter_radius / FILTER_TABLE_WIDTH as f32,
                );
                filter_table[y][x] = filter.evaluate(p);
            }
        }

        let area = (resolution.x * resolution.y) as usize;
        let pixels = {
            let mut pixels = Vec::with_capacity(area);
            for _ in 0..area {
                pixels.push(Pixel::default());
            }
            Mutex::new(pixels.into_boxed_slice())
        };

        return Film { resolution, filter_radius, filter_table, pixels };
    }

    pub fn sample_bbox(&self) -> BBox2u {
        BBox2u::new(
            Point2u::from((Point2f::default()             + HALF_PIXEL - self.filter_radius).floor()),
            Point2u::from((Point2f::from(self.resolution) - HALF_PIXEL + self.filter_radius).ceil()),
        )
    }

    pub fn iter(film: Arc<Film>) -> FilmTileIter {
        FilmTileIter::new(film)
    }

    fn get_film_tile(&self, bbox: BBox2u) -> FilmTile {
        let bbox = BBox2f::from(bbox);
        let min = (bbox.min - HALF_PIXEL - self.filter_radius).ceil();
        let max = (bbox.max + HALF_PIXEL + self.filter_radius).floor();
        let bbox = BBox2u::from(BBox2f::new(min, max));
        return FilmTile::new(bbox, self.filter_radius, self.filter_table);
    }

    fn pixel_offset(&self, Point2u { x, y }: Point2u) -> usize {
        let width = self.resolution.x;
        return (width * y + x) as usize;
    }

    pub fn merge_film_tile(&self, tile: FilmTile) {
        let mut pixels = self.pixels.lock().unwrap();
        for pixel in tile.bbox.iter() {
            let p = &tile.pixels[tile.pixel_offset(pixel.x, pixel.y)];
            (*pixels)[self.pixel_offset(pixel)].merge(p);
        }
    }

    /// Write an image file in plain ppm format.
    pub fn write_image_ppm<F>(&self, file: &mut F) where F: Write {
        let pixels = self.pixels.lock().unwrap();

        let header = format!("P3\n{} {}\n255\n", self.resolution.x, self.resolution.y);
        file.write_all(header.as_bytes()).unwrap();

        for p in pixels.iter() {
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
    filter_radius: f32,
    filter_table: FilterTable,
    pixels: Box<[Pixel]>,
}

impl FilmTile {
    fn new(bbox: BBox2u, filter_radius: f32, filter_table: FilterTable) -> FilmTile {
        let area = bbox.area() as usize;
        let pixels = {
            let mut pixels = Vec::with_capacity(area);
            for _ in 0..area {
                pixels.push(Pixel::default());
            }
            pixels.into_boxed_slice()
        };

        return FilmTile { bbox, filter_radius, filter_table, pixels };
    }

    fn pixel_offset(&self, mut x: u32, mut y: u32) -> usize {
        x -= self.bbox.min.x;
        y -= self.bbox.min.y;
        let width = self.bbox.max.x - self.bbox.min.x;
        return (width * y + x) as usize;
    }

    pub fn add_sample(&mut self, p_film: Point2f, sample: Spectrum) {
        let p_film = p_film - HALF_PIXEL;
        let min = Point2u::from((p_film - self.filter_radius).ceil());
        let max = Point2u::from((p_film + self.filter_radius).floor()) + 1;
        let min = min.max(self.bbox.min);
        let max = max.min(self.bbox.max);

        // filter table offsets
        let len = (self.filter_radius * 2.).ceil() as usize;
        let mut indices = [Vec::with_capacity(len), Vec::with_capacity(len)];
        for i in 0..2 {
            for p in min[i]..max[i] {
                let fi = ((p as f32 - p_film[i]) / self.filter_radius * FILTER_TABLE_WIDTH as f32).abs().floor() as usize;
                indices[i].push(cmp::min(fi, FILTER_TABLE_WIDTH - 1));
            }
        }
        let indices = indices;

        for y in min.y..max.y {
            for x in min.x..max.x {
                let weight = self.filter_table
                    [indices[1][(y - min.y) as usize]]
                    [indices[0][(x - min.x) as usize]];
                self.pixels[self.pixel_offset(x, y)].add_sample(sample, weight);
            }
        }
    }
}

pub struct FilmTileIter {
    film: Arc<Film>,
    n_tiles: Point2u,
    x: u32,
    y: u32,
    next_none: bool,
}

impl FilmTileIter {
    fn new(film: Arc<Film>) -> FilmTileIter {
        let n_tiles = (film.resolution + TILE_SIZE - 1) / TILE_SIZE;
        return FilmTileIter {
            film,
            n_tiles,
            x: 0,
            y: 0,
            next_none: n_tiles.x == 0 || n_tiles.y == 0,
        };
    }
}

impl Iterator for FilmTileIter {
    type Item = FilmTile;
    fn next(&mut self) -> Option<FilmTile> {
        if self.next_none {
            return None;
        }

        let min = Point2u::new(
            self.x * TILE_SIZE,
            self.y * TILE_SIZE,
        );
        let max = (min + TILE_SIZE).min(self.film.resolution);

        self.x += 1;
        if self.x >= self.n_tiles.x {
            self.x = 0;
            self.y += 1;
            if self.y >= self.n_tiles.y {
                self.next_none = true;
            }
        }

        return Some(self.film.get_film_tile(BBox2u::new(min, max)));
    }
}

#[derive(Default)]
struct Pixel {
    color: Spectrum,
    filter_weight_sum: f32,
}

impl Pixel {
    fn add_sample(&mut self, sample: Spectrum, filter_weight: f32) {
        self.color += sample * filter_weight;
        self.filter_weight_sum += filter_weight;
    }

    fn merge(&mut self, pixel: &Pixel) {
        self.color += pixel.color;
        self.filter_weight_sum += pixel.filter_weight_sum;
    }
}

fn gamma_correct(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1. / 2.4) - 0.055
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use bbox::BBox2u;
    use film::Film;
    use vector::Point2u;

    #[test]
    fn test_film_tile_iter() {
        let film = Arc::new(Film::new(Point2u::new(40, 40)));
        let iter = Film::iter(film);
        let tiles = [
            BBox2u { min: Point2u { x: 0, y: 0 }, max: Point2u { x: 16, y: 16 } }, BBox2u { min: Point2u { x: 16, y: 0 }, max: Point2u { x: 32, y: 16 } }, BBox2u { min: Point2u { x: 32, y: 0 }, max: Point2u { x: 40, y: 16 } },
            BBox2u { min: Point2u { x: 0, y: 16 }, max: Point2u { x: 16, y: 32 } }, BBox2u { min: Point2u { x: 16, y: 16 }, max: Point2u { x: 32, y: 32 } }, BBox2u { min: Point2u { x: 32, y: 16 }, max: Point2u { x: 40, y: 32 } },
            BBox2u { min: Point2u { x: 0, y: 32 }, max: Point2u { x: 16, y: 40 } }, BBox2u { min: Point2u { x: 16, y: 32 }, max: Point2u { x: 32, y: 40 } }, BBox2u { min: Point2u { x: 32, y: 32 }, max: Point2u { x: 40, y: 40 } },
        ];
        let mut n = 0;
        for tile in iter {
            assert_eq!(tile.bbox.min, tiles[n].min);
            assert_eq!(tile.bbox.max, tiles[n].max);
            n += 1;
        }
        assert_eq!(n, tiles.len());
    }
}
