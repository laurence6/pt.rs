extern crate rand;
use self::rand::Rng;
use self::rand::ThreadRng;

use common::Float;
use vector::{Point2u, Point2f};
use sampler::Sampler;

pub struct RandomSampler {
    // General sampler
    samplesPerPixel: usize,

    currentPixelSampleIndex: usize,

    sampleArray1D: Vec<Box<[Float]>>,
    sampleArray2D: Vec<Box<[Point2f]>>,

    // Next 1d array to be returned
    array1DOffset: usize,
    // Next 2d array to be returned
    array2DOffset: usize,

    // Random sampler
    rng: ThreadRng,
}

impl RandomSampler {
    pub fn New(samplesPerPixel: usize) -> RandomSampler {
        RandomSampler {
            samplesPerPixel: samplesPerPixel,

            currentPixelSampleIndex: 0,

            sampleArray1D: Vec::<Box<[Float]>>::new(),
            sampleArray2D: Vec::<Box<[Point2f]>>::new(),

            array1DOffset: 0,
            array2DOffset: 0,

            rng: rand::thread_rng(),
        }
    }
}

impl Sampler for RandomSampler {
    fn StartPixel(&mut self, _: Point2u) {
        self.currentPixelSampleIndex = 0;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        for i in 0..self.sampleArray1D.len() {
            for j in 0..self.sampleArray1D[i].len() {
                self.sampleArray1D[i][j] = self.rng.gen();
            }
        }

        for i in 0..self.sampleArray2D.len() {
            for j in 0..self.sampleArray2D[i].len() {
                self.sampleArray2D[i][j] = Point2f::New(self.rng.gen(), self.rng.gen());
            }
        }
    }

    fn StartNextSample(&mut self) -> bool {
        self.currentPixelSampleIndex += 1;
        self.array1DOffset = 0;
        self.array2DOffset = 0;
        return self.currentPixelSampleIndex < self.samplesPerPixel;
    }

    fn Get1D(&mut self) -> Float {
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);
        return self.rng.gen();
    }

    fn Get2D(&mut self) -> Point2f {
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);
        return Point2f::New(self.rng.gen(), self.rng.gen());
    }

    fn Req1DArray(&mut self, n: usize) {
        debug_assert_eq!(self.RoundCount(n), n);
        self.sampleArray1D.push(
            vec![0.0; n * self.samplesPerPixel]
            .into_boxed_slice()
        );
    }

    fn Req2DArray(&mut self, n: usize) {
        debug_assert_eq!(self.RoundCount(n), n);
        self.sampleArray2D.push(
            vec![Point2f::New(0.0, 0.0); n * self.samplesPerPixel]
            .into_boxed_slice()
        );
    }

    fn Get1DArray(&mut self, n: usize) -> Option<&[Float]> {
        if self.array1DOffset >= self.sampleArray1D.len() {
            return None;
        }

        debug_assert_eq!(self.sampleArray1D[self.array1DOffset].len(), n * self.samplesPerPixel);
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);

        let i0 = self.currentPixelSampleIndex * n;
        let i1 = i0 + n;
        let ret = Some(&self.sampleArray1D[self.array1DOffset][i0..i1]);

        self.array1DOffset += 1;

        return ret;
    }

    fn Get2DArray(&mut self, n: usize) -> Option<&[Point2f]> {
        if self.array2DOffset >= self.sampleArray2D.len() {
            return None;
        }

        debug_assert_eq!(self.sampleArray2D[self.array2DOffset].len(), n * self.samplesPerPixel);
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);

        let i0 = self.currentPixelSampleIndex * n;
        let i1 = i0 + n;
        let ret = Some(&self.sampleArray2D[self.array2DOffset][i0..i1]);

        self.array2DOffset += 1;

        return ret;
    }
}

#[cfg(test)]
mod sampler_random_test {
    #[test]
    fn TestRandomSampler() {
        use sampler::Sampler;
        use sampler_random::RandomSampler;
        use vector::Point2u;

        let samplesPerPixel = 7;
        let size = 13;
        let n1DArray = 3;
        let n2DArray = 5;

        let mut sampler = RandomSampler::New(samplesPerPixel);

        for _ in 0..n1DArray { sampler.Req1DArray(size); }
        for _ in 0..n2DArray { sampler.Req2DArray(size); }

        for i in 0..17 {
            for j in 0..19 {
                sampler.StartPixel(Point2u::New(i, j));

                let mut c = 0;
                loop {
                    for _ in 1..n1DArray {
                        assert!(sampler.Get1D() != 0.0, "Empty sample");
                    }
                    for _ in 0..n1DArray {
                        let array = sampler.Get1DArray(size).unwrap();
                        assert!(array.len() == size, "Incorrect size");

                        if c > 0 {
                            let s = array[size / 2];
                            assert!(s != 0.0, "Empty sample");
                        }
                    }
                    for _ in 0..n2DArray {
                        let array = sampler.Get2DArray(size).unwrap();
                        assert!(array.len() == size, "Incorrect size");

                        if c > 0 {
                            let s = array[size / 2];
                            assert!(s.X != 0.0 && s.Y != 0.0, "Empty sample");
                        }
                    }

                    c += 1;

                    if !sampler.StartNextSample() {
                        assert_eq!(c, samplesPerPixel);
                        break;
                    }
                }
            }
        }
    }
}
