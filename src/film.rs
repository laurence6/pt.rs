use vector::Point2u;

pub struct Film {
    pub Resolution: Point2u,
}

impl Film {
    pub fn New(resolution: Point2u) -> Film {
        return Film { Resolution: resolution };
    }
}
