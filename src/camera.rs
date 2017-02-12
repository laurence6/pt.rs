use film::Film;
use ray::Ray;
use vector::Point2f;

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub pFilm: Point2f,
    pub pLens: Point2f,
}

pub struct Camera {
    pub Film: Film,
}

impl Camera {
    pub fn New(film: Film) -> Camera {
        return Camera {
            Film: film,
        };
    }
}

impl Camera {
    /// Generate the world space ray corresponding to a sample position on the film plane.
    pub fn GenerateRay(sample: CameraSample) -> Ray {
        unimplemented!()
    }

    /// Generate the world space ray corresponding to a sample position on the film plane,
    /// and compute the information about the image area.
    pub fn GenerateRayDifferential() {

    }
}
