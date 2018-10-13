use std::sync::Arc;

use bbox::BBox3f;
use common::{EPSILON, gamma};
use interaction::Interaction;
use material::Material;
use ray::Ray;
use shape::Shape;
use vector::{Vector3f, Normal3f, Point3f, Point2f};

pub struct Triangle {
    vertices: [Point3f; 3],
    reverse_orientation: bool,

    material: Arc<Material>,
}

impl Triangle {
    pub fn new(vertices: [Point3f; 3], reverse_orientation: bool, material: Arc<Material>) -> Triangle {
        Triangle { vertices, reverse_orientation, material }
    }

    fn get_uv(&self) -> [Point2f; 3] {
        [
            Point2f::new(0., 0.),
            Point2f::new(1., 0.),
            Point2f::new(1., 1.),
        ]
    }
}

impl Shape for Triangle {
    fn bbox(&self) -> BBox3f {
        BBox3f::new(self.vertices[0], self.vertices[1])
            .union_point(self.vertices[2])
    }

    fn area(&self) -> f32 {
        0.5 * (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).length()
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let vs_o = self.vertices;
        let mut vs = vs_o.clone();

        // transform to ray coordinate system
        // ray.origin is at (0, 0, 0) & direction is (0, 0, 1)
        let max_axis = ray.direction.max_abs_axis();
        let (px, py, pz) = (max_axis.prev(), max_axis.next(), max_axis);
        let d = ray.direction.permute(px, py, pz);

        let (sx, sy, sz) = (-d.x/d.z, -d.y/d.z, 1./d.z);

        for p in vs.iter_mut() {
            *p -= Vector3f::from(ray.origin);
            *p = p.permute(px, py, pz);
            p.x += p.z * sx;
            p.y += p.z * sy;
        }

        // use edge function to test if (0, 0, _) is in the triangle
        let e0 = vs[1].x * vs[2].y - vs[1].y * vs[2].x;
        let e1 = vs[2].x * vs[0].y - vs[2].y * vs[0].x;
        let e2 = vs[0].x * vs[1].y - vs[0].y * vs[1].x;
        if e0 == 0. || e1 == 0. || e2 == 0. {
            // TODO: compute using f64
        }

        if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
            return false;
        }
        let det = e0 + e1 + e2;
        if det == 0. {
            return false;
        }

        for p in vs.iter_mut() {
            p.z *= sz;
        }
        let t_scaled = vs[0].z * e0 + vs[1].z * e1 + vs[2].z * e2;
        if (det < 0. && (t_scaled < ray.t_max * det || 0. <= t_scaled))
            || (det > 0. && (t_scaled <= 0. || ray.t_max * det < t_scaled))
        {
            return false;
        }

        let t = t_scaled / det;

        // ensure t > 0
        let max_x = Vector3f::new(vs[0].x, vs[1].x, vs[2].x).abs().max_component();
        let max_y = Vector3f::new(vs[0].y, vs[1].y, vs[2].y).abs().max_component();
        let max_z = Vector3f::new(vs[0].z, vs[1].z, vs[2].z).abs().max_component();
        let delta_x = max_x * gamma(5.);
        let delta_y = max_y * gamma(5.);
        let delta_z = max_z * gamma(3.);
        let delat_e = (gamma(2.) * max_x * max_y + max_x * delta_y + max_y * delta_x) * 2.;
        let max_e = Vector3f::new(e0, e1, e2).abs().max_component();
        let delta_t = 3.
                    * (gamma(3.) * max_e * max_z + max_z * delat_e + max_e * delta_z)
                    / det.abs();
        if t <= delta_t {
            return false;
        }

        return true;
    }

    fn intersect(&self, ray: &Ray) -> Option<(Interaction, f32)> {
        let vs_o = self.vertices;
        let mut vs = vs_o.clone();

        // transform to ray coordinate system
        // ray.origin is at (0, 0, 0) & direction is (0, 0, 1)
        let max_axis = ray.direction.max_abs_axis();
        let (px, py, pz) = (max_axis.prev(), max_axis.next(), max_axis);
        let d = ray.direction.permute(px, py, pz);

        let (sx, sy, sz) = (-d.x/d.z, -d.y/d.z, 1./d.z);

        for p in vs.iter_mut() {
            *p -= Vector3f::from(ray.origin);
            *p = p.permute(px, py, pz);
            p.x += p.z * sx;
            p.y += p.z * sy;
        }

        // use edge function to test if (0, 0, _) is in the triangle
        let e0 = vs[1].x * vs[2].y - vs[1].y * vs[2].x;
        let e1 = vs[2].x * vs[0].y - vs[2].y * vs[0].x;
        let e2 = vs[0].x * vs[1].y - vs[0].y * vs[1].x;
        if e0 == 0. || e1 == 0. || e2 == 0. {
            // TODO: compute using f64
        }

        if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
            return None;
        }
        let det = e0 + e1 + e2;
        if det == 0. {
            return None;
        }

        for p in vs.iter_mut() {
            p.z *= sz;
        }
        let t_scaled = vs[0].z * e0 + vs[1].z * e1 + vs[2].z * e2;
        if (det < 0. && (t_scaled < ray.t_max * det || 0. <= t_scaled))
            || (det > 0. && (t_scaled <= 0. || ray.t_max * det < t_scaled))
        {
            return None;
        }

        // barycentric coordinates
        let b0 = e0 / det;
        let b1 = e1 / det;
        let b2 = e2 / det;

        let t = t_scaled / det;

        // ensure t > 0
        let max_x = Vector3f::new(vs[0].x, vs[1].x, vs[2].x).abs().max_component();
        let max_y = Vector3f::new(vs[0].y, vs[1].y, vs[2].y).abs().max_component();
        let max_z = Vector3f::new(vs[0].z, vs[1].z, vs[2].z).abs().max_component();
        let delta_x = max_x * gamma(5.);
        let delta_y = max_y * gamma(5.);
        let delta_z = max_z * gamma(3.);
        let delat_e = (gamma(2.) * max_x * max_y + max_x * delta_y + max_y * delta_x) * 2.;
        let max_e = Vector3f::new(e0, e1, e2).abs().max_component();
        let delta_t = 3.
                    * (gamma(3.) * max_e * max_z + max_z * delat_e + max_e * delta_z)
                    / det.abs();
        if t <= delta_t {
            return None;
        }

        let p = vs_o[0] * b0
              + vs_o[1] * b1
              + vs_o[2] * b2;
        let p_abs_sum = (vs_o[0] * b0).abs()
                      + (vs_o[1] * b1).abs()
                      + (vs_o[2] * b2).abs();
        let p_err = Vector3f::from(p_abs_sum) * gamma(7.);

        let uv = self.get_uv();
        let duv02 = uv[0] - uv[2];
        let duv12 = uv[1] - uv[2];
        let dp02 = vs_o[0] - vs_o[2];
        let dp12 = vs_o[1] - vs_o[2];
        let det = duv02[0] * duv12[1] - duv02[1] * duv12[0];
        let (dpdu, dpdv) = if det.abs() < EPSILON {
            (vs_o[2] - vs_o[0]).cross(vs_o[1] - vs_o[0])
                .normalize()
                .construct_coordinate_system()
        } else {
            let dpdu = (dp02 *  duv12[1] - dp12 * duv02[1]) / det;
            let dpdv = (dp02 * -duv12[0] + dp12 * duv02[0]) / det;
            (dpdu, dpdv)
        };

        let mut n = Normal3f::from(dp02.cross(dp12).normalize());
        if self.reverse_orientation {
            n *= -1.;
        }

        return Some((
            Interaction {
                p,
                p_err,
                n,
                dpdu,
                dpdv,
                wo: -ray.direction,
                shape: None,
            },
            t,
        ));
    }

    fn material(&self) -> Arc<Material> {
        self.material.clone()
    }

    fn sample(&self, sample: Point2f) -> Interaction {
        let vs_o = self.vertices;
        let (b0, b1, b2) = {
            let sample_0 = sample[0].sqrt();
            let b0 = 1. - sample_0;
            let b1 = sample[1] * sample_0;
            let b2 = 1. - b0 - b1;
            (b0, b1, b2)
        };

        let p = vs_o[0] * b0
              + vs_o[1] * b1
              + vs_o[2] * b2;
        let p_abs_sum = (vs_o[0] * b0).abs()
                      + (vs_o[1] * b1).abs()
                      + (vs_o[2] * b2).abs();
        let p_err = Vector3f::from(p_abs_sum) * gamma(7.);

        let mut n = Normal3f::from((vs_o[1] - vs_o[0]).cross(vs_o[2] - vs_o[0]).normalize());
        if self.reverse_orientation {
            n *= -1.;
        }

        return Interaction {
            p,
            p_err,
            n,
            ..Default::default()
        }
    }
}
