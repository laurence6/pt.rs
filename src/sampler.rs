use common::Float;
use vector::{Point2u, Point2f};
use camera::CameraSample;

pub trait Sampler {
    /// Set current pixel. Reset sample number.
    fn start_pixel(&mut self, p: Point2u);
    /// Start next sample of current pixel.
    /// Return false if requested samples per pixel have been generated.
    fn start_next_sample(&mut self) -> bool;
    /// Return next 1 dimension of current sample.
    fn get_1d(&mut self) -> Float;
    /// Return next 2 dimensions of current sample.
    fn get_2d(&mut self) -> Point2f;
    /// Request an array of n samples with 1 dimension.
    fn req_1d_array(&mut self, usize);
    /// Request an array of n samples with 2 dimensions.
    fn req_2d_array(&mut self, usize);
    /// Get an array of samples with 1 dimension.
    fn get_1d_array(&mut self, usize) -> Option<&[Float]>;
    /// Get an array of samples with 2 dimensions.
    fn get_2d_array(&mut self, usize) -> Option<&[Point2f]>;
    /// Round to a better size of array.
    fn round_count(&self, n: usize) -> usize {
        n
    }

    fn get_camera_sample(&mut self, p_raster: Point2u) -> CameraSample {
        let p_film = Point2f::from(p_raster) + self.get_2d();
        let p_lens = self.get_2d();
        return CameraSample {
            p_film: p_film,
            p_lens: p_lens,
        };
    }
}

pub trait GlobalSampler : Sampler {
    /// Return index to the sample in the overall set of samples based on current pixel and sample index.
    fn get_index_for_sample(&mut self, usize) -> usize;
    /// Return sample value for the given dimension of the indexth sample in the overall set of samples.
    fn sample_dimension(&self, index: usize, d: usize) -> Float;
}
