use camera::CameraSample;
use vector::{Point2i, Point2f};

pub trait Sampler : Clone + Send {
    /// Set current pixel. Reset sample number.
    fn start_pixel(&mut self, p: Point2i);

    /// Start next sample of current pixel.
    /// Return false if requested samples per pixel have been generated.
    fn start_next_sample(&mut self) -> bool;

    /// Return next 1 dimension of current sample.
    fn get_1d(&mut self) -> f32;

    /// Return next 2 dimensions of current sample.
    fn get_2d(&mut self) -> Point2f;

    fn get_camera_sample(&mut self, p_raster: Point2i) -> CameraSample {
        CameraSample {
            p_film: Point2f::from(p_raster) + self.get_2d(),
        }
    }
}
