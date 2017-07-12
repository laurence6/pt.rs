use std::rc::Rc;

use bbox::BBox3f;
use common::{PI, gamma, quadratic};
use interaction::Interaction;
use material::Material;
use ray::Ray;
use shape::Shape;
use vector::{Vector3f, Normal3f, Point3f};

pub struct Sphere {
    center: Point3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    fn world_to_local(&self, p: Point3f) -> Point3f {
        p + -self.center
    }

    fn local_to_world(&self, p: Point3f) -> Point3f {
        p + self.center
    }

    // return dpdu, dpdv
    fn compute_partial_derivative(&self, Point3f { x, y, z }: Point3f) -> (Vector3f, Vector3f) {
        let dpdu = Vector3f::new(
            -2. * PI * y,
            2. * PI * x,
            0.,
        );
        let r_sin_theta = (x * x + y * y).sqrt(); // r * sin(theta) = (r^2 - z^2)^0.5 or (x^2 + y^2)^0.5
        let dpdv = Vector3f::new(
            PI * z * x/r_sin_theta,
            PI * z * y/r_sin_theta,
            PI * -r_sin_theta,
        );
        return (dpdu, dpdv);
    }

    fn compute_interaction(&self, ray: &Ray, t: f32) -> Interaction {
        let mut p = ray.position(t);
        p *= self.radius / Vector3f::from(p).length();

        let p_err = Vector3f::from(p).abs() * gamma(5.);

        let n = {
            let n = Normal3f::from(Vector3f::from(p).normalize());
            if Vector3f::from(ray.origin).length() < self.radius {
                -n
            } else {
                n
            }
        };

        let (dpdu, dpdv) = self.compute_partial_derivative(p);

        return Interaction {
            p: self.local_to_world(p),
            p_err,
            n,
            dpdu,
            dpdv,
            wo: -ray.direction,
            shape: None,
        };
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

    fn intersect(&self, ray: &Ray) -> Option<(Interaction, f32)> {
        let mut ray = ray.clone();
        ray.origin = self.world_to_local(ray.origin);

        let o = ray.origin;
        let d = ray.direction;
        let (ox, oy, oz) = (o.x as f64, o.y as f64, o.z as f64);
        let (dx, dy, dz) = (d.x as f64, d.y as f64, d.z as f64);
        let r = self.radius as f64;
        let a = dx * dx + dy * dy + dz * dz;
        let b = 2. * (dx * ox + dy * oy + dz * oz);
        let c = ox * ox + oy * oy + oz * oz - r * r;

        let s = quadratic(a, b, c);
        if s.is_none() {
            return None;
        }

        let (t0, t1) = s.unwrap();
        let (t0, t1) = (t0 as f32, t1 as f32);
        if 0. < t0 && t0 < ray.t_max {
            return Some((self.compute_interaction(&ray, t0), t0));
        }
        if 0. < t1 && t1 < ray.t_max {
            return Some((self.compute_interaction(&ray, t1), t1));
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
