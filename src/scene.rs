use bbox::BBox;
use ray::Ray;
use interaction::SurfaceInteraction;
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
        unimplemented!()
    }

    pub fn Intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        unimplemented!()
    }
}
