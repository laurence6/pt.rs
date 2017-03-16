use bbox::BBox;
use ray::Ray;
use interaction::Interaction;
use light::Light;
use primitive::Primitive;

pub struct Scene {
    lights: Vec<Box<Light>>,
    aggregate: Box<Primitive>,
}

impl Scene {
    pub fn BBox(&self) -> BBox {
        self.aggregate.BBox()
    }

    pub fn IntersectP(&self, ray: &Ray) -> bool {
        self.aggregate.IntersectP(ray)
    }

    pub fn Intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.aggregate.Intersect(ray)
    }
}
