use std::cmp;
use std::io::Write;
use std::sync::{Mutex};

use bbox::{BBox2i, BBox2f};
use common::clamp;
use filter::Filter;
use spectrum::Spectrum;
use vector::{Vector2i, Vector2f, Point2u, Point2i, Point2f};

const FILTER_TABLE_WIDTH: usize = 16;
type FilterTable = [[f32; FILTER_TABLE_WIDTH]; FILTER_TABLE_WIDTH];

const TILE_SIZE: i32 = 16;

const HALF_PIXEL: Vector2f = Vector2f { x: 0.5, y: 0.5 };

pub struct Film {
    pub resolution: Point2i,
    filter_radius: f32,
    filter_table: FilterTable,
    pixels: Mutex<Box<[Pixel]>>,
}

impl Film {
    pub fn new<T>(resolution: Point2u, filter: T) -> Film where T: Filter {
        let resolution = Point2i::from(resolution);

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

    pub fn sample_bbox(&self) -> BBox2i {
        BBox2i::new(
            Point2i::from((Point2f::default()             + HALF_PIXEL - self.filter_radius).floor()),
            Point2i::from((Point2f::from(self.resolution) - HALF_PIXEL + self.filter_radius).ceil()),
        )
    }

    pub fn iter(&self) -> FilmTileIter {
        FilmTileIter::new(self.sample_bbox())
    }

    pub fn get_film_tile(&self, bbox: BBox2i) -> FilmTile {
        let bbox = BBox2f::from(bbox);
        let min = Point2i::from((bbox.min - HALF_PIXEL - self.filter_radius).ceil());
        let max = Point2i::from((bbox.max + HALF_PIXEL + self.filter_radius).floor());
        let min = min.max(Point2i::default());
        let max = max.min(self.resolution);
        let bbox = BBox2i::new(min, max);
        return FilmTile::new(bbox, self.filter_radius, self.filter_table);
    }

    fn pixel_offset(&self, Point2i { x, y }: Point2i) -> usize {
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
                color[i] = clamp(gamma_correct(color[i] / p.filter_weight_sum) * 255. + 0.5, 0., 255.).round();
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
    bbox: BBox2i,
    filter_radius: f32,
    filter_table: FilterTable,
    pixels: Box<[Pixel]>,
}

impl FilmTile {
    fn new(bbox: BBox2i, filter_radius: f32, filter_table: FilterTable) -> FilmTile {
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

    fn pixel_offset(&self, mut x: i32, mut y: i32) -> usize {
        x -= self.bbox.min.x;
        y -= self.bbox.min.y;
        let width = self.bbox.max.x - self.bbox.min.x;
        return (width * y + x) as usize;
    }

    pub fn add_sample(&mut self, p_film: Point2f, sample: Spectrum) {
        let p_film = p_film - HALF_PIXEL;
        let min = Point2i::from((p_film - self.filter_radius).ceil());
        let max = Point2i::from((p_film + self.filter_radius).floor()) + 1;
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
    bbox: BBox2i,
    n_tiles: Vector2i,
    i: Point2i,
}

impl FilmTileIter {
    fn new(sample_bbox: BBox2i) -> FilmTileIter {
        let n_tiles = (sample_bbox.diagonal() + TILE_SIZE - 1) / TILE_SIZE;
        return FilmTileIter {
            bbox: sample_bbox,
            n_tiles,
            i: Point2i::default(),
        };
    }
}

impl Iterator for FilmTileIter {
    type Item = BBox2i;
    fn next(&mut self) -> Option<BBox2i> {
        loop {
            if self.i.y >= self.n_tiles.y {
                return None;
            } else if self.i.x >= self.n_tiles.x {
                self.i.x = 0;
                self.i.y += 1;
            } else {
                let min = self.bbox.min + self.i * TILE_SIZE;
                let max = (min + TILE_SIZE).min(self.bbox.max);
                let tile = Some(BBox2i::new(min, max));
                self.i.x += 1;
                return tile;
            }
        }
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
    use bbox::BBox2;
    use film::Film;
    use filter::GaussianFilter;
    use vector::Point2;

    #[test]
    fn test_film_tile_iter() {
        let film = Film::new(Point2::new(0, 40), GaussianFilter::new(0., 0.));
        let mut n = 0;
        for tile in film.iter() {
            n += 1;
        }
        assert_eq!(n, 0);

        let film = Film::new(Point2::new(40, 0), GaussianFilter::new(0., 0.));
        let mut n = 0;
        for tile in film.iter() {
            n += 1;
        }
        assert_eq!(n, 0);

        let film = Film::new(Point2::new(40, 40), GaussianFilter::new(0., 0.));
        let tiles = [
            BBox2 { min: Point2 { x: 0, y: 0 }, max: Point2 { x: 16, y: 16 } }, BBox2 { min: Point2 { x: 16, y: 0 }, max: Point2 { x: 32, y: 16 } }, BBox2 { min: Point2 { x: 32, y: 0 }, max: Point2 { x: 40, y: 16 } },
            BBox2 { min: Point2 { x: 0, y: 16 }, max: Point2 { x: 16, y: 32 } }, BBox2 { min: Point2 { x: 16, y: 16 }, max: Point2 { x: 32, y: 32 } }, BBox2 { min: Point2 { x: 32, y: 16 }, max: Point2 { x: 40, y: 32 } },
            BBox2 { min: Point2 { x: 0, y: 32 }, max: Point2 { x: 16, y: 40 } }, BBox2 { min: Point2 { x: 16, y: 32 }, max: Point2 { x: 32, y: 40 } }, BBox2 { min: Point2 { x: 32, y: 32 }, max: Point2 { x: 40, y: 40 } },
        ];
        let mut n = 0;
        for tile in film.iter() {
            assert_eq!(tile.min, tiles[n].min);
            assert_eq!(tile.max, tiles[n].max);
            n += 1;
        }
        assert_eq!(n, tiles.len());
    }
}
