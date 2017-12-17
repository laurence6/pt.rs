extern crate rand;
use self::rand::{Rng, StdRng};

use sampler::Sampler;
use vector::{Point2u, Point2f};

#[derive(Clone)]
pub struct RandomSampler {
    // General sampler
    samples_per_pixel: u32,
    current_pixel_sample_index: u32,
    // Random sampler
    rng: StdRng,
}

impl RandomSampler {
    pub fn new(samples_per_pixel: u32) -> RandomSampler {
        RandomSampler {
            samples_per_pixel,
            current_pixel_sample_index: 0,
            rng: StdRng::new().unwrap(),
        }
    }
}

impl Sampler for RandomSampler {
    fn start_pixel(&mut self, _: Point2u) {
        self.current_pixel_sample_index = 0;
    }

    fn start_next_sample(&mut self) -> bool {
        self.current_pixel_sample_index += 1;
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
}
