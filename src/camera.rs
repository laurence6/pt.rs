use vector::Vector;
use ray::Ray;
use film::Film;

struct Camera {
    pub Film: Film,
}

impl Camera {
    pub fn New(film: Film) -> Camera {
        return Camera {
            Film: film,
        };
    }

    pub fn GenerateRay() {
    }
}
