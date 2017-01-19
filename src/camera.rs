use vector::Vector;
use vector::Point2f;
use ray::Ray;
use film::Film;

pub struct Camera {
    pub Film: Film,
}

/// Records the position on Film that Camera should generate corresponding ray.
pub struct CameraSample {
    pub pFilm: Point2f,
    pub pLens: Point2f,
}

impl Camera {
    pub fn New(film: Film) -> Camera {
        return Camera {
            Film: film,
        };
    }

    pub fn GenerateRay(sample: CameraSample) -> Ray {
        unimplemented!()
    }
}
