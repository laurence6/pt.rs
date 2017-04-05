use vector::Point2u;

pub struct Film {
    pub resolution: Point2u,
}

impl Film {
    pub fn new(resolution: Point2u) -> Film {
        Film { resolution: resolution }
    }
}
