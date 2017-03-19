use film::Film;
use ray::Ray;
use vector::{Point2f, Vector3f, Point3f};
use matrix::Transform;
use bbox::BBox2f;
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
    pub fn New(cameraToWorld: Transform, screenWindow: BBox2f, film: Film, fov: Float) -> PerspectiveCamera {
        let cameraToScreen = Transform::Perspective(fov, 1.0e-2, 1000.0);

        let screenToRaster = Transform::Scale(Vector3f::New(film.Resolution.X as Float, film.Resolution.Y as Float, 1.0))
                           * Transform::Scale(Vector3f::New(1.0 / (screenWindow.Max.X - screenWindow.Min.Y), 1.0 / (screenWindow.Max.Y - screenWindow.Min.Y), 1.0))
                           * Transform::Translate(Vector3f::New(-screenWindow.Min.X, -screenWindow.Max.Y, 0.0));
        let rasterToScreen = screenToRaster.Inverse();

        let rasterToCamera = cameraToScreen.Inverse() * rasterToScreen;

        return PerspectiveCamera {
            film: film,
            cameraToWorld: cameraToWorld,

            cameraToScreen: cameraToScreen,
            rasterToCamera: rasterToCamera,

            screenToRaster: screenToRaster,
            rasterToScreen: rasterToScreen,
        };
    }
}

impl Camera for PerspectiveCamera {
    fn GenerateRay(&self, sample: &CameraSample) -> Ray {
        let pFilm = Point3f::New(sample.pFilm.X, sample.pFilm.Y, 0.0);
        let pCamera = self.rasterToCamera.ApplyPoint(pFilm);

        let ray = Ray {
            Direction: pCamera,
            ..Default::default()
        };

        return self.cameraToWorld.ApplyRay(&ray);
    }
}
