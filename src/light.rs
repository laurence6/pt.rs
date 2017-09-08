use std::sync::Arc;

use bbox::BBox3f;
use container::Container;
use interaction::Interaction;
use material::Material;
use ray::Ray;
use scene::Scene;
use shape::Shape;
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

pub struct AreaLight<S> where S: Shape {
    l: Spectrum,
    shape: S,
}

impl<S> AreaLight<S> where S: Shape {
    pub fn new(l: Spectrum, shape: S) -> AreaLight<S> {
        AreaLight::<S> { l, shape }
    }
}

impl<S> Shape for AreaLight<S> where S: Shape {
    fn bbox(&self) -> BBox3f {
        self.shape.bbox()
    }

    fn area(&self) -> f32 {
        self.shape.area()
    }

    fn is_light(&self) -> bool {
        true
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.shape.intersect_p(ray)
    }

    fn intersect(&self, ray: &Ray) -> Option<(Interaction, f32)> {
        self.shape.intersect(ray)
    }

    fn material(&self) -> Arc<Material> {
        self.shape.material()
    }

    fn sample(&self, sample: Point2f) -> Interaction {
        self.shape.sample(sample)
    }

    fn sample_ref(&self, ref_i: &Interaction, sample: Point2f) -> Interaction {
        self.shape.sample_ref(ref_i, sample)
    }

    fn pdf(&self, ref_i: &Interaction, wi: Vector3f) -> f32 {
        self.shape.pdf(ref_i, wi)
    }

    fn l(&self, i: &Interaction, w: Vector3f) -> Spectrum {
        if Vector3f::from(i.n).dot(w) > 0. {
            self.l
        } else {
            Spectrum::default()
        }
    }
}

impl<S> Light for AreaLight<S> where S: Shape {
    fn sample_li(&self, ref_i: &Interaction, sample: Point2f) -> (Vector3f, Spectrum, f32, VisibilityTester) {
        let p_shape = self.shape.sample_ref(ref_i, sample);
        let wi = (p_shape.p - ref_i.p).normalize();
        let l = self.l(&p_shape, -wi);
        let pdf = self.shape.pdf(ref_i, wi);
        let vis = VisibilityTester {
            p0: ref_i.clone(),
            p1: p_shape,
        };
        return (wi, l, pdf, vis);
    }
}
