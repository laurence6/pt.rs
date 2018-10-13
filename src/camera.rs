use bbox::BBox2f;
use ray::Ray;
use transform::Transform;
use vector::{Vector3f, Point3f, Point2u, Point2f};

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub p_film: Point2f,
}

pub trait Camera : Clone + Send {
    /// Generate the world space ray corresponding to a sample position on the film plane.
    fn generate_ray(&self, &CameraSample) -> Ray;
}

#[derive(Clone)]
pub struct PerspectiveCamera {
    camera_to_world: Transform,

    raster_to_camera: Transform,
    camera_to_screen: Transform,
    screen_to_raster: Transform,
    raster_to_screen: Transform,
}

impl PerspectiveCamera {
    pub fn new(camera_to_world: Transform, camera_to_screen: Transform, screen_window: BBox2f, film_res: Point2u) -> PerspectiveCamera {
        let screen_to_raster = Transform::scale(
                                   film_res.x as f32,
                                   film_res.y as f32,
                                   1.,
                               )  // 3. scale to raster space
                             * Transform::scale(
                                   1. / (screen_window.max.x - screen_window.min.x),
                                   1. / (screen_window.min.y - screen_window.max.y),
                                   1.,
                               )  // 2. scale to normalized device coordinate space
                             * Transform::translate(
                                   -screen_window.min.x,
                                   -screen_window.max.y,
                                   0.,
                               ); // 1. move upper-left corner of the screen to the origin
        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        return PerspectiveCamera {
            camera_to_world,

            raster_to_camera,
            camera_to_screen,
            screen_to_raster,
            raster_to_screen,
        };
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: &CameraSample) -> Ray {
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.);
        let p_camera = self.raster_to_camera.apply(&p_film);

        let ray = Ray {
            direction: Vector3f::from(p_camera).normalize(),
            ..Default::default()
        };

        return self.camera_to_world.apply(&ray);
    }
}
