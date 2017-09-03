use bbox::BBox3f;
use container::Container;
use interaction::Interaction;
use scene::Scene;
use spectrum::Spectrum;
use vector::{Vector3f, Point3f, Point2f};

pub trait Light : Sync + Send {
    fn pre_process(&mut self, scene_bbox: BBox3f) {}

    /// sample_li takes a world space point and returns incident direction (direction radiance is arriving from), the radiance arriving at that point, pdf, and VisibilityTester.
    fn sample_li(&self, i: &Interaction, sample: Point2f) -> (Vector3f, Spectrum, f32, VisibilityTester);
}

pub struct VisibilityTester {
    p0: Interaction,
    p1: Interaction,
}

impl VisibilityTester {
    pub fn unoccluded<C: Container>(&self, scene: &Scene<C>) -> bool {
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
            w_light: w_light.normalize(),
            world_center: Point3f::default(),
            world_radius: f32::default(),
        }
    }
}

impl Light for DistantLight {
    fn pre_process(&mut self, scene_bbox: BBox3f) {
        let (center, radius) = scene_bbox.bounding_sphere();
        self.world_center = center;
        self.world_radius = radius;
    }

    fn sample_li(&self, i: &Interaction, sample: Point2f) -> (Vector3f, Spectrum, f32, VisibilityTester) {
        let vis = VisibilityTester {
            p0: i.clone(),
            p1: Interaction {
                p: i.p + self.w_light * (2. * self.world_radius), // A point outside the scene
                ..Default::default()
            },
        };

        return (self.w_light, self.l, 1., vis);
    }
}
