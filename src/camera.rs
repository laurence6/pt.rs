use film::Film;
use ray::Ray;
use vector::{Point2f, Vector3f, Point3f};
use transform::Transform;
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
    camera_to_world: Transform,

    camera_to_screen: Transform,
    raster_to_camera: Transform,

    screen_to_raster: Transform,
    raster_to_screen: Transform,
}

impl PerspectiveCamera {
    pub fn new(camera_to_world: Transform, screen_window: BBox2f, film: &Film, fov: Float) -> PerspectiveCamera {
        let camera_to_screen = Transform::perspective(fov, 1.0e-2, 1000.0);

        let screen_to_raster = Transform::scale(Vector3f::new(film.resolution.x as Float, film.resolution.y as Float, 1.0))
                             * Transform::scale(Vector3f::new(1.0 / (screen_window.max.x - screen_window.min.y), 1.0 / (screen_window.max.y - screen_window.min.y), 1.0))
                             * Transform::translate(Vector3f::new(-screen_window.min.x, -screen_window.max.y, 0.0));
        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        return PerspectiveCamera {
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
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.raster_to_camera.apply(&p_film);

        let ray = Ray {
            direction: Vector3f::from(p_camera),
            ..Default::default()
        };

        return self.camera_to_world.apply(&ray);
    }
}
