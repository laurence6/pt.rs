use std::rc::Rc;

use common::Float;
use vector::{Vector, Point3f};
use spectrum::Spectrum;
use interaction::{Interaction, BaseInteraction};
use scene::Scene;

pub trait Light {
    /// SampleLi takes a world space point and returns the radiance arriving at that point,
    /// incident direction, and VisibilityTester.
    fn SampleLi(&self, Rc<Interaction>) -> (Spectrum, Vector, VisibilityTester);
    fn PreProcess(&mut self, &Scene) {}
}

pub struct VisibilityTester {
    p0: Rc<Interaction>,
    p1: Rc<Interaction>,
}

impl VisibilityTester {
    fn Unoccluded(&self, scene: &Scene) -> bool {
        unimplemented!()
    }
}

pub struct DistantLight {
    l: Spectrum,
    wLight: Vector,
    worldCenter: Point3f,
    worldRadius: Float,
}

impl Light for DistantLight {
    fn SampleLi(&self, r: Rc<Interaction>) -> (Spectrum, Vector, VisibilityTester) {
        // A point outside the scene
        let pOuside = r.GetP() + self.wLight * (2.0 * self.worldRadius);
        let p1 = BaseInteraction {
            p: pOuside,
            .. Default::default()
        };
        let vis = VisibilityTester {
            p0: r,
            p1: Rc::new(p1),
        };

        return (self.l, self.wLight, vis);
    }

    fn PreProcess(&mut self, scene: &Scene) {
        let (center, radius) = scene.BBox().BoundingSphere();
        self.worldCenter = center;
        self.worldRadius = radius;
    }
}
