extern crate rand;
use self::rand::{Rng, StdRng};

use sampler::Sampler;
use vector::{Point2u, Point2f};

#[derive(Clone)]
pub struct RandomSampler {
    // General sampler
    samples_per_pixel: usize,

    current_pixel_sample_index: usize,

    sample_array_1d: Vec<Box<[f32]>>,
    sample_array_2d: Vec<Box<[Point2f]>>,

    // Next 1d array to be returned
    array_1d_offset: usize,
    // Next 2d array to be returned
    array_2d_offset: usize,

    // Random sampler
    rng: StdRng,
}

impl RandomSampler {
    pub fn new(samples_per_pixel: usize) -> RandomSampler {
        RandomSampler {
            samples_per_pixel,

            current_pixel_sample_index: 0,

            sample_array_1d: Vec::new(),
            sample_array_2d: Vec::new(),

            array_1d_offset: 0,
            array_2d_offset: 0,

            rng: StdRng::new().unwrap(),
        }
    }
}

impl Sampler for RandomSampler {
    fn start_pixel(&mut self, _: Point2u) {
        self.current_pixel_sample_index = 0;
        self.array_1d_offset = 0;
        self.array_2d_offset = 0;

        for i in 0..self.sample_array_1d.len() {
            for j in 0..self.sample_array_1d[i].len() {
                self.sample_array_1d[i][j] = self.rng.gen();
            }
        }

        for i in 0..self.sample_array_2d.len() {
            for j in 0..self.sample_array_2d[i].len() {
                self.sample_array_2d[i][j] = Point2f::new(self.rng.gen(), self.rng.gen());
            }
        }
    }

    fn start_next_sample(&mut self) -> bool {
        self.current_pixel_sample_index += 1;
        self.array_1d_offset = 0;
        self.array_2d_offset = 0;
        return self.current_pixel_sample_index < self.samples_per_pixel;
    }

    fn get_1d(&mut self) -> f32 {
        debug_assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        return self.rng.gen();
    }

    fn get_2d(&mut self) -> Point2f {
        debug_assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        return Point2f::new(self.rng.gen(), self.rng.gen());
    }

    fn req_1d_array(&mut self, n: usize) {
        debug_assert_eq!(self.round_count(n), n);
        self.sample_array_1d
            .push(vec![0.; n * self.samples_per_pixel].into_boxed_slice());
    }

    fn req_2d_array(&mut self, n: usize) {
        debug_assert_eq!(self.round_count(n), n);
        self.sample_array_2d
            .push(vec![Point2f::new(0., 0.); n * self.samples_per_pixel].into_boxed_slice());
    }

    fn get_1d_array(&mut self, n: usize) -> Option<&[f32]> {
        if self.array_1d_offset >= self.sample_array_1d.len() {
            return None;
        }

        debug_assert_eq!(self.sample_array_1d[self.array_1d_offset].len(), n * self.samples_per_pixel);
        debug_assert!(self.current_pixel_sample_index < self.samples_per_pixel);

        let i0 = self.current_pixel_sample_index * n;
        let i1 = i0 + n;
        let ret = Some(&self.sample_array_1d[self.array_1d_offset][i0..i1]);

        self.array_1d_offset += 1;

        return ret;
    }

    fn get_2d_array(&mut self, n: usize) -> Option<&[Point2f]> {
        if self.array_2d_offset >= self.sample_array_2d.len() {
            return None;
        }

        debug_assert_eq!(self.sample_array_2d[self.array_2d_offset].len(), n * self.samples_per_pixel);
        debug_assert!(self.current_pixel_sample_index < self.samples_per_pixel);

        let i0 = self.current_pixel_sample_index * n;
        let i1 = i0 + n;
        let ret = Some(&self.sample_array_2d[self.array_2d_offset][i0..i1]);

        self.array_2d_offset += 1;

        return ret;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_random_sampler() {
        use sampler::Sampler;
        use sampler_random::RandomSampler;
        use vector::Point2u;

        let samples_per_pixel = 7;
        let size = 13;
        let n_1d_array = 3;
        let n_2d_array = 5;

        let mut sampler = RandomSampler::new(samples_per_pixel);

        for _ in 0..n_1d_array { sampler.req_1d_array(size); }
        for _ in 0..n_2d_array { sampler.req_2d_array(size); }

        for i in 0..17 {
            for j in 0..19 {
                sampler.start_pixel(Point2u::new(i, j));

                let mut c = 0;
                loop {
                    for _ in 1..n_1d_array {
                        assert!(sampler.get_1d() != 0., "Empty sample");
                    }
                    for _ in 0..n_1d_array {
                        let array = sampler.get_1d_array(size).unwrap();
                        assert!(array.len() == size, "Incorrect size");

                        if c > 0 {
                            let s = array[size / 2];
                            assert!(s != 0., "Empty sample");
                        }
                    }
                    for _ in 0..n_2d_array {
                        let array = sampler.get_2d_array(size).unwrap();
                        assert!(array.len() == size, "Incorrect size");

                        if c > 0 {
                            let s = array[size / 2];
                            assert!(s.x != 0. && s.y != 0., "Empty sample");
                        }
                    }

                    c += 1;

                    if !sampler.start_next_sample() {
                        assert_eq!(c, samples_per_pixel);
                        break;
                    }
                }
            }
        }
    }
}
