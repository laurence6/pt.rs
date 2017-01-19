use common::Float;
use vector::Point2i;
use vector::Point2f;
use camera::CameraSample;

pub trait Sampler {
    fn StartPixel(&mut self, p: Point2i);
    fn StartNextSample(&mut self) -> bool;
    fn Req1D(&mut self, usize);
    fn Req2D(&mut self, usize);
    fn Get1D(&mut self, usize) -> Option<Float>;
    fn Get2D(&mut self, usize) -> Option<Point2f>;
    fn GetCameraSample(&self) -> CameraSample;

    fn GetIndexForSample(&self, sampleNum: usize) -> usize;
    fn SampleDimension(&self, index: usize, d: usize) -> Float;
}

pub struct HaltonSampler {
    samplesPerPixel: usize,

    currentPixel: Point2i,
    currentPixelSampleIndex: usize,

    //sampleArray1DSizes: Vec<usize>,
    //sampleArray2DSizes: Vec<usize>,
    sampleArray1D: Vec<Vec<Float>>,
    sampleArray2D: Vec<Vec<Point2f>>,

    // Next array to be returned
    array1DOffset: usize,
    array2DOffset: usize,

    dimension: usize, // Next dimension
    sampleIndex: usize, // Point to sample in current pixel

    arrayStartDim: usize, // Default = 4 (x, y, u, v, (u, u)...)
    arrayEndDim: usize,
}

impl Sampler for HaltonSampler {
    fn StartPixel(&mut self, p: Point2i) {
        self.currentPixel = p;
        self.currentPixelSampleIndex = 0;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        self.dimension = 0;
        self.sampleIndex = self.GetIndexForSample(0);

        self.arrayEndDim = self.arrayStartDim + self.sampleArray1D.len() + 2 * self.sampleArray2D.len();
    }

    fn StartNextSample(&mut self) -> bool {
        self.currentPixelSampleIndex += 1;
        self.array1DOffset = 0;
        self.array2DOffset = 0;
        return self.currentPixelSampleIndex < self.samplesPerPixel;
    }

    fn Req1D(&mut self, n: usize) {
        self.sampleArray1D.push(Vec::<Float>::with_capacity(n));
    }

    fn Req2D(&mut self, n: usize) {
        self.sampleArray2D.push(Vec::<Point2f>::with_capacity(n));
    }

    fn Get1D(&mut self, n: usize) -> Option<Float> {
        if self.array1DOffset == self.sampleArray1D.len() {
            return None;
        }
        let ret = Some(self.sampleArray1D[self.array1DOffset][self.currentPixelSampleIndex * n]);
        self.array1DOffset += 1;
        return ret;
    }

    fn Get2D(&mut self, n: usize) -> Option<Point2f> {
        if self.array2DOffset == self.sampleArray2D.len() {
            return None;
        }
        let ret = Some(self.sampleArray2D[self.array2DOffset][self.currentPixelSampleIndex * n]);
        self.array2DOffset += 1;
        return ret;
    }

    fn GetCameraSample(&self) -> CameraSample {
        unimplemented!()
    }

    fn GetIndexForSample(&self, sampleNum: usize) -> usize {
        unimplemented!()
    }

    fn SampleDimension(&self, index: usize, d: usize) -> Float {
        unimplemented!()
    }
}
