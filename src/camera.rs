use film::Film;
use ray::Ray;
use vector::{Point2f, Vector, ZERO_VECTOR};
use matrix::Transform;
use common::Float;

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub pFilm: Point2f,
    pub pLens: Point2f,
}

pub trait Camera {
    /// Generate the world space ray corresponding to a sample position on the film plane.
    fn GenerateRay(&self, &CameraSample) -> Ray;

    ///// Generate the world space ray corresponding to a sample position on the film plane,
    ///// and compute the information about the image area.
    //fn GenerateRayDifferential(&self);
}

pub struct PerspectiveCamera {
    film: Film,
    cameraToWorld: Transform,

    cameraToScreen: Transform,
    rasterToCamera: Transform,

    screenToRaster: Transform,
    rasterToScreen: Transform,
}

impl PerspectiveCamera {
    pub fn New(cameraToWorld: Transform, film: Film, fov: Float) -> PerspectiveCamera {
        unimplemented!()
        //return PerspectiveCamera {
        //    film: film,
        //    cameraToWorld: cameraToWorld,
        // };
    }
}

impl Camera for PerspectiveCamera {
    fn GenerateRay(&self, sample: &CameraSample) -> Ray {
        let pFilm = Vector::New(sample.pFilm.X, sample.pFilm.Y, 0.0);
        let pCamera = self.rasterToCamera.ApplyPoint(pFilm);

        let ray = Ray::New(ZERO_VECTOR, pCamera);

        // TODO: FOV

        return self.cameraToWorld.ApplyRay(&ray);
    }
}
