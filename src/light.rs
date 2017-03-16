use common::Float;
use vector::{Vector, Point3f};
use spectrum::Spectrum;
use interaction::Interaction;
use scene::Scene;

pub trait Light {
    /// SampleLi takes a world space point and returns the radiance arriving at that point,
    /// incident direction, and VisibilityTester.
    fn SampleLi(&self, &Interaction) -> (Spectrum, Vector, VisibilityTester);
    fn PreProcess(&mut self, &Scene) {}
}

pub struct VisibilityTester {
    p0: Interaction,
    p1: Interaction,
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
    fn SampleLi(&self, i: &Interaction) -> (Spectrum, Vector, VisibilityTester) {
        let mut p1 = Interaction::default();
        // A point outside the scene
        p1.p = i.p + self.wLight * (2.0 * self.worldRadius);
        let vis = VisibilityTester {
            p0: i.clone(),
            p1: p1,
        };

        return (self.l, self.wLight, vis);
    }

    fn PreProcess(&mut self, scene: &Scene) {
        let (center, radius) = scene.BBox().BoundingSphere();
        self.worldCenter = center;
        self.worldRadius = radius;
    }
}
