use common::Float;
use vector::Point2i;
use vector::Point2f;
use camera::CameraSample;

pub trait Sampler {
    /// Set current pixel. Reset sample number.
    fn StartPixel(&mut self, p: Point2i);
    /// Start next sample of current pixel.
    /// Return false if requested samples per pixel have been generated.
    fn StartNextSample(&mut self) -> bool;
    /// Return next 1 dimension of current sample.
    fn Get1D(&mut self) -> Float;
    /// Return next 2 dimensions of current sample.
    fn Get2D(&mut self) -> Point2f;
    /// Request an array of n samples with 1 dimension.
    fn Req1DArray(&mut self, usize) {
        unimplemented!()
    }
    /// Request an array of n samples with 2 dimensions.
    fn Req2DArray(&mut self, usize) {
        unimplemented!()
    }
    /// Get an array of samples with 1 dimension.
    fn Get1DArray(&mut self) -> Option<&[Float]> {
        unimplemented!()
    }
    /// Get an array of samples with 2 dimensions.
    fn Get2DArray(&mut self) -> Option<&[Point2f]> {
        unimplemented!()
    }
    /// Round to a better size of array.
    fn RoundCount(n: usize) -> usize {
        return n;
    }

    fn GetCameraSample(&mut self, pRaster: Point2i) -> CameraSample {
        let pFilm = Point2f::From(pRaster) + self.Get2D();
        let pLens = self.Get2D();
        return CameraSample {
            pFilm: pFilm,
            pLens: pLens,
        };
    }
}

pub trait GlobalSampler : Sampler {
    /// Return index to the sample in the overall set of samples based on current pixel and sample index.
    fn GetIndexForSample(&self, sampleNum: usize) -> usize;
    /// Return sample value for the given dimension of the indexth sample in the overall set of samples.
    fn SampleDimension(&self, index: usize, d: usize) -> Float;
}

pub struct HaltonSampler {
    // General sampler
    samplesPerPixel: usize,

    currentPixel: Point2i,
    currentPixelSampleIndex: usize,

    sampleArray1DSizes: Vec<usize>,
    sampleArray2DSizes: Vec<usize>,
    sampleArray1D: Vec<Vec<Float>>,
    sampleArray2D: Vec<Vec<Point2f>>,

    /// Next 1d array to be returned
    array1DOffset: usize,
    /// Next 2d array to be returned
    array2DOffset: usize,

    // Global sampler
    /// Next dimension
    dimension: usize,
    /// Index of sample in current pixel
    intervalSampleIndex: usize,

    arrayStartDim: usize, // Default = 4 (x, y, u, v, (u, v)...)
    arrayEndDim: usize,
}

impl HaltonSampler {
    pub fn New() -> HaltonSampler {
        unimplemented!()
    }
}

impl Sampler for HaltonSampler {
    fn StartPixel(&mut self, p: Point2i) {
        self.currentPixel = p;
        self.currentPixelSampleIndex = 0;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        self.dimension = 0;
        self.intervalSampleIndex = self.GetIndexForSample(0);
        self.arrayEndDim = self.arrayStartDim
            + self.sampleArray1D.len()
            + self.sampleArray2D.len() * 2;

        // Compute 1D array samples
        for i in 0..self.sampleArray1D.len() {
            let nSample = self.sampleArray1DSizes[i] * self.samplesPerPixel;
            for j in 0..nSample {
                let index = self.GetIndexForSample(j);
                self.sampleArray1D[i][j] = self.SampleDimension(index, self.arrayStartDim + i);
            }
        }

        // Compute 2D array samples
        let mut dim = self.arrayStartDim + self.sampleArray1D.len();
        for i in 0..self.sampleArray2D.len() {
            let nSample = self.sampleArray2DSizes[i] * self.samplesPerPixel;
            for j in 0..nSample {
                let index = self.GetIndexForSample(j);
                self.sampleArray2D[i][j].X = self.SampleDimension(index, dim);
                self.sampleArray2D[i][j].Y = self.SampleDimension(index, dim + 1);
            }
            dim += 2;
        }
        debug_assert_eq!(dim, self.arrayEndDim);
    }

    fn StartNextSample(&mut self) -> bool {
        self.currentPixelSampleIndex += 1;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        self.dimension = 0;
        self.intervalSampleIndex =
            self.GetIndexForSample(self.currentPixelSampleIndex + 1);

        return self.currentPixelSampleIndex < self.samplesPerPixel;
    }

    fn Get1D(&mut self) -> Float {
        if self.arrayStartDim <= self.dimension && self.dimension <= self.arrayEndDim {
            self.dimension = self.arrayEndDim;
        }
        self.dimension += 1;
        return self.SampleDimension(self.intervalSampleIndex, self.dimension);
    }

    fn Get2D(&mut self) -> Point2f {
        if self.arrayStartDim <= self.dimension + 1 && self.dimension + 1 <= self.arrayEndDim {
            self.dimension = self.arrayEndDim;
        }
        let p = Point2f::New(
            self.SampleDimension(self.intervalSampleIndex, self.dimension),
            self.SampleDimension(self.intervalSampleIndex, self.dimension + 1),
        );
        self.dimension += 2;
        return p;
    }

    fn Req1DArray(&mut self, n: usize) {
        self.sampleArray1DSizes.push(n);
        self.sampleArray1D.push(Vec::<Float>::with_capacity(n));
    }

    fn Req2DArray(&mut self, n: usize) {
        self.sampleArray2DSizes.push(n);
        self.sampleArray2D.push(Vec::<Point2f>::with_capacity(n));
    }

    fn Get1DArray(&mut self, n: usize) -> Option<&[Float]> {
        if self.array1DOffset == self.sampleArray1D.len() {
            return None;
        }
        let ret = {
            let i0 = self.currentPixelSampleIndex * n;
            let i1 = i0 + n;
            Some(&self.sampleArray1D[self.array1DOffset][i0..i1])
        };
        self.array1DOffset += 1;
        return ret;
    }

    fn Get2DArray(&mut self, n: usize) -> Option<&[Point2f]> {
        if self.array2DOffset == self.sampleArray2D.len() {
            return None;
        }
        let ret = {
            let i0 = self.currentPixelSampleIndex * n;
            let i1 = i0 + n;
            Some(&self.sampleArray2D[self.array2DOffset][i0..i1])
        };
        self.array2DOffset += 1;
        return ret;
    }
}

impl GlobalSampler for HaltonSampler {
    fn GetIndexForSample(&self, sampleNum: usize) -> usize {
        unimplemented!()
    }

    fn SampleDimension(&self, index: usize, d: usize) -> Float {
        unimplemented!()
    }
}
