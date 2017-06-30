use std::ops;

use bbox::BBox3f;
use common::Float;
use matrix::Matrix;
use ray::Ray;
use vector::{Vector3f, Normal3f, Point3f};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    m: Matrix,
    m_inv: Matrix,
}

impl From<Matrix> for Transform {
    fn from(m: Matrix) -> Transform {
        Transform {
            m,
            m_inv: m.inverse(),
        }
    }
}

impl From<[[Float; 4]; 4]> for Transform {
    fn from(m: [[Float; 4]; 4]) -> Transform {
        let m = Matrix::from(m);
        return Transform {
            m,
            m_inv: m.inverse(),
        };
    }
}

impl Transform {
    pub fn translate(x: Float, y: Float, z: Float) -> Transform {
        Transform {
            m: Matrix::new(
                1., 0., 0.,  x,
                0., 1., 0.,  y,
                0., 0., 1.,  z,
                0., 0., 0., 1.,
            ),
            m_inv: Matrix::new(
                1., 0., 0., -x,
                0., 1., 0., -y,
                0., 0., 1., -z,
                0., 0., 0., 1.,
            ),
        }
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Transform {
        Transform {
            m: Matrix::new(
                 x, 0., 0., 0.,
                0.,  y, 0., 0.,
                0., 0.,  z, 0.,
                0., 0., 0., 1.,
            ),
            m_inv: Matrix::new(
                1./x,   0.,   0., 0.,
                  0., 1./y,   0., 0.,
                  0.,   0., 1./z, 0.,
                  0.,   0.,   0., 1.,
            ),
        }
    }

    /// Compute perspective transformation from field-of-view angel, distance to a near z plane and a far z plane.
    pub fn perspective(fov: Float, n: Float, f: Float) -> Transform {
        let p = Matrix::new(
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., f / (f-n), -f * n / (f-n),
            0., 0., 1., 0.,
        );
        let inv_tan_ang = 1. / (fov.to_radians() / 2.).tan();
        return Transform::scale(inv_tan_ang, inv_tan_ang, 1.)
             * Transform::from(p);
    }

    /// Compute look-at transformation from camera position, a point camera looks at and up direction in world space coordinates. Return camera_to_world transformation.
    pub fn look_at(pos: Point3f, look: Point3f, up: Vector3f) -> Transform {
        let d = (look - pos).normalize();
        let left = up.normalize().cross(d).normalize();
        let up = d.cross(left); // make sure it's orthogonal to other two vectors

        let camera_to_world = Matrix::new(
            left.x, up.x, d.x, pos.x,
            left.y, up.y, d.y, pos.y,
            left.z, up.z, d.z, pos.z,
                0.,   0.,  0.,    1.,
        );

        return Transform {
            m: camera_to_world,
            m_inv: camera_to_world.inverse(),
        };
    }

    fn transpose(&self) -> Transform {
        Transform {
            m: self.m.transpose(),
            m_inv: self.m_inv.transpose(),
        }
    }

    pub fn inverse(&self) -> Transform {
        Transform { m: self.m_inv, m_inv: self.m }
    }

    fn rotate_x(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = Matrix::new(
            1.,        0.,         0., 0.,
            0., cos_theta, -sin_theta, 0.,
            0., sin_theta,  cos_theta, 0.,
            0.,        0.,         0., 1.,
        );
        return Transform {
            m,
            m_inv: m.transpose(),
        };
    }

    fn rotate_y(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = Matrix::new(
             cos_theta, 0., sin_theta, 0.,
                    0., 1.,        0., 0.,
            -sin_theta, 0., cos_theta, 0.,
                    0., 0.,        0., 1.,
        );
        return Transform {
            m,
            m_inv: m.transpose(),
        };
    }

    fn rotate_z(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = Matrix::new(
            cos_theta, -sin_theta, 0., 0.,
            sin_theta,  cos_theta, 0., 0.,
                   0.,         0., 1., 0.,
                   0.,         0., 0., 1.,
        );
        return Transform {
            m,
            m_inv: m.transpose(),
        };
    }

    pub fn apply<T>(&self, t: &T) -> T where T: Transformable {
        t.transform(self)
    }
}

impl PartialEq for Transform {
    fn eq(&self, t: &Transform) -> bool {
        self.m == t.m
    }
}

impl ops::Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, t: Transform) -> Transform {
        Transform {
            m: self.m * t.m,
            m_inv: t.m_inv * self.m_inv,
        }
    }
}

fn compute_sin_cos_in_degree(deg: Float) -> (Float, Float) {
    (deg.to_radians().sin(), deg.to_radians().cos())
}

pub trait Transformable {
    fn transform(&self, &Transform) -> Self;
}

impl Transformable for Vector3f {
    fn transform(&self, t: &Transform) -> Vector3f {
        Vector3f::new(
            t.m[0][0] * self.x + t.m[0][1] * self.y + t.m[0][2] * self.z,
            t.m[1][0] * self.x + t.m[1][1] * self.y + t.m[1][2] * self.z,
            t.m[2][0] * self.x + t.m[2][1] * self.y + t.m[2][2] * self.z,
        )
    }
}

impl Transformable for Normal3f {
    fn transform(&self, t: &Transform) -> Normal3f {
        Normal3f::new(
            t.m_inv[0][0] * self.x + t.m_inv[1][0] * self.y + t.m_inv[2][0] * self.z,
            t.m_inv[0][1] * self.x + t.m_inv[1][1] * self.y + t.m_inv[2][1] * self.z,
            t.m_inv[0][2] * self.x + t.m_inv[1][2] * self.y + t.m_inv[2][2] * self.z,
        )
    }
}

impl Transformable for Point3f {
    fn transform(&self, t: &Transform) -> Point3f {
        let xp = t.m[0][0] * self.x + t.m[0][1] * self.y + t.m[0][2] * self.z + t.m[0][3];
        let yp = t.m[1][0] * self.x + t.m[1][1] * self.y + t.m[1][2] * self.z + t.m[1][3];
        let zp = t.m[2][0] * self.x + t.m[2][1] * self.y + t.m[2][2] * self.z + t.m[2][3];
        let wp = t.m[3][0] * self.x + t.m[3][1] * self.y + t.m[3][2] * self.z + t.m[3][3];
        debug_assert!(wp != 0.);

        let p = Point3f::new(xp, yp, zp);
        if wp == 1. {
            return p;
        } else {
            return p / wp;
        }
    }
}

impl Transformable for Ray {
    fn transform(&self, t: &Transform) -> Ray {
        Ray {
            origin: t.apply(&self.origin),
            direction: t.apply(&self.direction),
            t_max: self.t_max,
        }
    }
}

impl Transformable for BBox3f {
    fn transform(&self, t: &Transform) -> BBox3f {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use matrix::Matrix;
    use transform::Transform;
    use vector::{Vector3f, Point3f};

    #[test]
    fn test_look_at() {
        let la = Transform::look_at(
            Point3f::new(1., 1., 0.),
            Point3f::new(2., 1., 0.),
            Vector3f::new(0., 0., 1.),
        );
        let la_e = Transform::from(Matrix::new(
            0., 0., 1., 1.,
            1., 0., 0., 1.,
            0., 1., 0., 0.,
            0., 0., 0., 1.,
        ));
        assert_eq!(la, la_e);
    }

    #[test]
    fn test_transform_point3f() {
        let t = Transform::scale(3., 1.5, 1.)
              * Transform::translate(1., 2., 3.);
        let p = Point3f::new(0., 0., 0.);
        assert_eq!(t.apply(&p), Point3f::new(3., 3., 3.));
    }

    #[test]
    fn test_transform_mul() {
        let t1 = Transform::scale(1., 2., 3.);
        let t2 = Transform::translate(3., 2., 1.);
        let t = t1 * t2;
        let t_e = Transform::from(Matrix::new(
            1., 0., 0., 3.,
            0., 2., 0., 4.,
            0., 0., 3., 3.,
            0., 0., 0., 1.,
        ));
        assert_eq!(t, t_e);
        assert_eq!(t.m, t_e.m);
        assert_eq!(t.m_inv, t_e.m_inv);
    }
}
