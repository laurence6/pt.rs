use std::rc::Rc;

use bbox::BBox3f;
use container::Container;
use interaction::Interaction;
use ray::Ray;
use shape::{Shape, intersect};

pub struct Simple {
    shapes: Box<[Rc<Shape>]>,
    bbox: BBox3f,
}

impl Simple {
    pub fn new(shapes: Box<[Rc<Shape>]>) -> Simple {
        let bbox = BBox3f::bbox_of_shapes(&shapes);
        return Simple {
            shapes: shapes,
            bbox: bbox,
        };
    }
}

impl Container for Simple {
    fn bbox(&self) -> BBox3f {
        self.bbox
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        if self.bbox.intersect(ray).is_none() {
            return false;
        }

        for shape in self.shapes.iter() {
            if shape.intersect_p(ray) {
                return true;
            }
        }

        return false;
    }

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        if self.bbox.intersect(ray).is_none() {
            return None;
        }

        let mut ray = ray.clone();
        let mut interaction = None;

        for shape in self.shapes.iter() {
            let i = intersect(shape, &mut ray);
            if i.is_some() {
                interaction = i;
            }
        }

        return interaction;
    }
}
