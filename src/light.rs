use interaction::Interaction;
use ray::Ray;
use scene::Scene;
use spectrum::Spectrum;
use vector::{Vector3f, Point3f, Point2f};

pub trait Light {
    fn pre_process(&mut self, &Scene) {}

    fn le(&self, ray: &Ray) -> Spectrum {
        Spectrum::default()
    }

    /// sample_li takes a world space point and returns the radiance arriving at that point,
    /// incident direction (direction radiance is arriving from), and VisibilityTester.
    fn sample_li(&self, i: &Interaction, sample: Point2f) -> (Spectrum, Vector3f, VisibilityTester);
}

pub struct VisibilityTester {
    p0: Interaction,
    p1: Interaction,
}

impl VisibilityTester {
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.intersect_p(&self.p0.spawn_ray_to(self.p1.p))
    }
}

pub struct DistantLight {
    l: Spectrum,
    w_light: Vector3f,
    world_center: Point3f,
    world_radius: f32,
}

impl DistantLight {
    pub fn new(l: Spectrum, w_light: Vector3f) -> DistantLight {
        DistantLight {
            l,
            w_light,
            world_center: Point3f::default(),
            world_radius: f32::default(),
        }
    }
}

impl Light for DistantLight {
    fn pre_process(&mut self, scene: &Scene) {
        let (center, radius) = scene.bbox().bounding_sphere();
        self.world_center = center;
        self.world_radius = radius;
    }

    fn sample_li(&self, i: &Interaction, sample: Point2f) -> (Spectrum, Vector3f, VisibilityTester) {
        let vis = VisibilityTester {
            p0: i.clone(),
            p1: Interaction {
                p: i.p + self.w_light * (2. * self.world_radius), // A point outside the scene
                ..Default::default()
            },
        };

        return (self.l, self.w_light, vis);
    }
}
