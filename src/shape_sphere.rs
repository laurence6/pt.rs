use std::rc::Rc;

use bbox::BBox3f;
use common::{Float, EPSILON};
use interaction::Interaction;
use material::Material;
use ray::Ray;
use shape::Shape;
use vector::{Normal3f, Point3f};

pub struct Sphere {
    center: Point3f,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Point3f, radius: Float) -> Sphere {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn bbox(&self) -> BBox3f {
        let min = Point3f::new(self.center.x - self.radius, self.center.y - self.radius, self.center.z - self.radius);
        let max = Point3f::new(self.center.x + self.radius, self.center.y + self.radius, self.center.z + self.radius);
        return BBox3f::new(min, max);
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<(Interaction, Float)> {
        let to = ray.origin - self.center;
        let b = ray.direction.dot(to);
        let c = to.dot(to) - self.radius.powi(2);
        let d = b.powi(2) - c;

        if d < 0. {
            return None;
        }

        let d = d.sqrt();

        let t = -b - d;
        if EPSILON < t && t < ray.t_max {
            let p = ray.position(t);
            let n = Normal3f::from((p - self.center).normalize());
            return Some((
                Interaction { p, n, ..Default::default() },
                t,
            ));
        }

        let t = -b + d;
        if EPSILON < t && t < ray.t_max {
            let p = ray.position(t);
            let n = Normal3f::from((self.center - p).normalize());
            return Some((
                Interaction { p, n, ..Default::default() },
                t,
            ));
        }

        return None;
    }

    fn material(&self) -> Rc<Material> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use ray::Ray;
    use shape::Shape;
    use shape_sphere::Sphere;
    use vector::{Vector3f, Normal3f, Point3f};

    #[test]
    fn test_intersect() {
        let sphere = Sphere::new(Point3f::new(2., 2., 2.), 1.);

        let ray = Ray {
            origin: Point3f::new(2., 0., 2.),
            direction: Vector3f::new(0., 1., 0.),
            t_max: 10.,
        };
        let (interaction, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(interaction.p, Point3f::new(2., 1., 2.));
        assert_eq!(interaction.n, Normal3f::new(0., -1., 0.));

        let ray = Ray {
            origin: Point3f::new(2., 4., 2.),
            direction: Vector3f::new(0., -1., 0.),
            t_max: 10.,
        };
        let (interaction, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(interaction.p, Point3f::new(2., 3., 2.));
        assert_eq!(interaction.n, Normal3f::new(0., 1., 0.));

        let ray = Ray {
            origin: Point3f::new(2., 2., 2.),
            direction: Vector3f::new(0., 1., 0.),
            t_max: 10.,
        };
        let (interaction, _) = sphere.intersect(&ray).unwrap();
        assert_eq!(interaction.p, Point3f::new(2., 3., 2.));
        assert_eq!(interaction.n, Normal3f::new(0., -1., 0.));

        let ray = Ray {
            origin: Point3f::new(0., 0., 2.),
            direction: Vector3f::new(7.109401, 6.9729223, 2.91366),
            t_max: 10.,
        };
        let (interaction, _) = sphere.intersect(&ray).unwrap();

        let ray = Ray {
            origin: Point3f::new(2., 0., 2.),
            direction: Vector3f::new(0., 1., 1.).normalize(),
            t_max: 10.,
        };
        let interaction = sphere.intersect(&ray);
        if let Some((interaction, _)) = interaction {
            panic!("interaction found: p:{:?} n:{:?}", interaction.p, interaction.n);
        }
    }
}
