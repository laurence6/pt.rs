use vector::Vector;

pub struct BBox {
    pub Min: Vector,
    pub Max: Vector,
}

impl BBox {
    pub fn New(min: Vector, max: Vector) -> BBox {
        return BBox { Min: min, Max: max };
    }

    pub fn Intersect(&self) -> bool {
        unimplemented!()
    }

    pub fn IntersectP(&self) -> bool {
        unimplemented!()
    }
}
