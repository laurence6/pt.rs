use film::Film;
use ray::Ray;
use vector::Point2f;
use matrix::Transform;

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub pFilm: Point2f,
    pub pLens: Point2f,
}

pub trait Camera {
    /// Generate the world space ray corresponding to a sample position on the film plane.
    fn GenerateRay(&self, sample: &CameraSample) -> Ray;

    /// Generate the world space ray corresponding to a sample position on the film plane,
    /// and compute the information about the image area.
    fn GenerateRayDifferential(&self);
}

pub struct CameraBase {
    cameraToWorld: Transform,
    film: Film,
}

impl CameraBase {
    pub fn New(cameraToWorld: Transform, film: Film) -> CameraBase {
        return CameraBase {
            cameraToWorld: cameraToWorld,
            film: film,
        };
    }
}

impl CameraBase {
    pub fn GenerateRay(&self, sample: &CameraSample) -> Ray {
        unimplemented!()
    }

    pub fn GenerateRayDifferential(&self) {

    }
}
