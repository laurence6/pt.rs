use film::Film;
use ray::Ray;
use vector::{Point2f, Vector3f, Point3f};
use matrix::Transform;
use bbox::BBox2f;
use common::Float;

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub p_film: Point2f,
    pub p_lens: Point2f,
}

pub trait Camera {
    /// Generate the world space ray corresponding to a sample position on the film plane.
    fn generate_ray(&self, &CameraSample) -> Ray;
}

pub struct PerspectiveCamera {
    film: Film,
    camera_to_world: Transform,

    camera_to_screen: Transform,
    raster_to_camera: Transform,

    screen_to_raster: Transform,
    raster_to_screen: Transform,
}

impl PerspectiveCamera {
    pub fn new(camera_to_world: Transform, screen_window: BBox2f, film: Film, fov: Float) -> PerspectiveCamera {
        let camera_to_screen = Transform::Perspective(fov, 1.0e-2, 1000.0);

        let screen_to_raster = Transform::Scale(Vector3f::New(film.Resolution.X as Float, film.Resolution.Y as Float, 1.0))
                             * Transform::Scale(Vector3f::New(1.0 / (screen_window.max.X - screen_window.min.Y), 1.0 / (screen_window.max.Y - screen_window.min.Y), 1.0))
                             * Transform::Translate(Vector3f::New(-screen_window.min.X, -screen_window.max.Y, 0.0));
        let raster_to_screen = screen_to_raster.Inverse();

        let raster_to_camera = camera_to_screen.Inverse() * raster_to_screen;

        return PerspectiveCamera {
            film: film,
            camera_to_world: camera_to_world,

            camera_to_screen: camera_to_screen,
            raster_to_camera: raster_to_camera,

            screen_to_raster: screen_to_raster,
            raster_to_screen: raster_to_screen,
        };
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: &CameraSample) -> Ray {
        let p_film = Point3f::New(sample.p_film.X, sample.p_film.Y, 0.0);
        let p_camera = self.raster_to_camera.ApplyPoint(p_film);

        let ray = Ray {
            Direction: p_camera,
            ..Default::default()
        };

        return self.camera_to_world.ApplyRay(&ray);
    }
}
