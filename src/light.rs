use common::Float;
use interaction::Interaction;
use scene::Scene;
use spectrum::Spectrum;
use vector::{Vector3f, Point3f};

pub trait Light {
    /// sample_li takes a world space point and returns the radiance arriving at that point,
    /// incident direction, and VisibilityTester.
    fn sample_li(&self, &Interaction) -> (Spectrum, Vector3f, VisibilityTester);
    fn pre_process(&mut self, &Scene) {}
}

pub struct VisibilityTester {
    p0: Interaction,
    p1: Interaction,
}

impl VisibilityTester {
    fn unoccluded(&self, scene: &Scene) -> bool {
        scene.intersect_p(&self.p0.spawn_ray_to(self.p1.p))
    }
}

pub struct DistantLight {
    l: Spectrum,
    w_light: Vector3f,
    world_center: Point3f,
    world_radius: Float,
}

impl Light for DistantLight {
    fn sample_li(&self, i: &Interaction) -> (Spectrum, Vector3f, VisibilityTester) {
        let mut p1 = Interaction::default();
        // A point outside the scene
        p1.p = i.p + self.w_light * (2. * self.world_radius);
        let vis = VisibilityTester {
            p0: i.clone(),
            p1: p1,
        };

        return (self.l, self.w_light, vis);
    }

    fn pre_process(&mut self, scene: &Scene) {
        let (center, radius) = scene.bbox().bounding_sphere();
        self.world_center = center;
        self.world_radius = radius;
    }
}
