use std::rc::Rc;

use bbox::BBox3f;
use common::{EPSILON, gamma};
use interaction::Interaction;
use material::Material;
use ray::Ray;
use shape::Shape;
use vector::{Vector3f, Normal3f, Point3f, Point2f};

pub struct Triangle {
    vertices: [Point3f; 3],

    material: Rc<Material>,
}

impl Triangle {
    pub fn new(vertices: [Point3f; 3], material: Rc<Material>) -> Triangle {
        Triangle { vertices, material }
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

    fn intersect_p(&self, ray: &Ray) -> bool {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<(Interaction, f32)> {
        let vs_o = self.vertices;
        let mut vs = vs_o.clone();

        // transform to ray coordinate system
        // ray.origin is at (0, 0, 0) & direction is (0, 0, 1)
        let max_axis = ray.direction.max_axis();
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

        let b0 = e0 / det;
        let b1 = e1 / det;
        let b2 = e2 / det;
        let t = t_scaled / det;

        let p = vs_o[0] * b0
              + vs_o[1] * b1
              + vs_o[2] * b2;
        let x_abs_sum = (b0 * vs_o[0].x).abs() + (b1 * vs_o[1].x).abs() + (b2 * vs_o[2].x).abs();
        let y_abs_sum = (b0 * vs_o[0].y).abs() + (b1 * vs_o[1].y).abs() + (b2 * vs_o[2].y).abs();
        let z_abs_sum = (b0 * vs_o[0].z).abs() + (b1 * vs_o[1].z).abs() + (b2 * vs_o[2].z).abs();
        let p_err = Vector3f::new(x_abs_sum, y_abs_sum, z_abs_sum) * gamma(7.);

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

        let n = Normal3f::from(dp02.cross(dp12).normalize());

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

    fn material(&self) -> Rc<Material> {
        self.material.clone()
    }
}
