use common::Float;
use vector::{Point2u, Point2f};
use camera::CameraSample;

pub trait Sampler {
    /// Set current pixel. Reset sample number.
    fn StartPixel(&mut self, p: Point2u);
    /// Start next sample of current pixel.
    /// Return false if requested samples per pixel have been generated.
    fn StartNextSample(&mut self) -> bool;
    /// Return next 1 dimension of current sample.
    fn Get1D(&mut self) -> Float;
    /// Return next 2 dimensions of current sample.
    fn Get2D(&mut self) -> Point2f;
    /// Request an array of n samples with 1 dimension.
    fn Req1DArray(&mut self, u32);
    /// Request an array of n samples with 2 dimensions.
    fn Req2DArray(&mut self, u32);
    /// Get an array of samples with 1 dimension.
    fn Get1DArray(&mut self, u32) -> Option<&[Float]>;
    /// Get an array of samples with 2 dimensions.
    fn Get2DArray(&mut self, u32) -> Option<&[Point2f]>;
    /// Round to a better size of array.
    fn RoundCount(&self, n: u32) -> u32 {
        n
    }

    fn GetCameraSample(&mut self, pRaster: Point2u) -> CameraSample {
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
    fn GetIndexForSample(&mut self, u32) -> u32;
    /// Return sample value for the given dimension of the indexth sample in the overall set of samples.
    fn SampleDimension(&self, index: u32, d: u32) -> Float;
}
