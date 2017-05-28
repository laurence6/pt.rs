use std::ops;

use common::Float;
use vector::{Vector3f, Normal3f, Point3f};
use ray::Ray;
use bbox::BBox3f;
use matrix::Matrix;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    m: Matrix,
    m_inv: Matrix,
}

impl From<Matrix> for Transform {
    fn from(m: Matrix) -> Transform {
        Transform {
            m: m,
            m_inv: m.inverse(),
        }
    }
}

impl From<[[Float; 4]; 4]> for Transform {
    fn from(m: [[Float; 4]; 4]) -> Transform {
        let mat = Matrix::from(m);
        return Transform {
            m: mat,
            m_inv: mat.inverse(),
        };
    }
}

impl Transform {
    pub fn translate(Vector3f { x, y, z }: Vector3f) -> Transform {
        Transform {
            m: Matrix::new(
                1.0, 0.0, 0.0,   x,
                0.0, 1.0, 0.0,   y,
                0.0, 0.0, 1.0,   z,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix::new(
                1.0, 0.0, 0.0,  -x,
                0.0, 1.0, 0.0,  -y,
                0.0, 0.0, 1.0,  -z,
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn scale(Vector3f { x, y, z }: Vector3f) -> Transform {
        Transform {
            m: Matrix::new(
                  x, 0.0, 0.0, 0.0,
                0.0,   y, 0.0, 0.0,
                0.0, 0.0,   z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix::new(
                1.0/x, 0.0,   0.0,   0.0,
                0.0,   1.0/y, 0.0,   0.0,
                0.0,   0.0,   1.0/z, 0.0,
                0.0,   0.0,   0.0,   1.0,
            ),
        }
    }

    /// Compute perspective transformation from field-of-view angel, distance to near a near z
    /// plane and a far z plane.
    pub fn perspective(fov: Float, n: Float, f: Float) -> Transform {
        let p = Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, f / (f-n), -f * n / (f-n),
            0.0, 0.0, 1.0, 0.0,
        );
        let inv_tan_ang = 1.0 / (fov.to_radians() / 2.0).tan();
        return Transform::scale(Vector3f::new(inv_tan_ang, inv_tan_ang, 1.0))
             * Transform::from(p);
    }

    /// Compute look-at transformation from camera position, a point camera looks at and up
    /// direction in world space coordinates.
    fn look_at(pos: Vector3f, look: Vector3f, up: Vector3f) -> Transform {
        let d = (look - pos).normalize();
        let up = up.normalize();
        let left = up.cross(d).normalize();
        let up = d.cross(left);

        let camera_to_world = Matrix::new(
            left.x, up.x, d.x, pos.x,
            left.y, up.y, d.y, pos.y,
            left.z, up.z, d.z, pos.z,
               0.0,  0.0, 0.0,   1.0,
        );

        return Transform { m: camera_to_world.inverse(), m_inv: camera_to_world };
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
            1.0,       0.0,        0.0, 0.0,
            0.0, cos_theta, -sin_theta, 0.0,
            0.0, sin_theta,  cos_theta, 0.0,
            0.0,       0.0,        0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
    }

    fn rotate_y(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = Matrix::new(
             cos_theta, 0.0, sin_theta, 0.0,
                  0.0,  1.0,       0.0, 0.0,
            -sin_theta, 0.0, cos_theta, 0.0,
                  0.0,  0.0,       0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
    }

    fn rotate_z(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = Matrix::new(
            cos_theta, -sin_theta, 0.0, 0.0,
            sin_theta,  cos_theta, 0.0, 0.0,
                  0.0,        0.0, 1.0, 0.0,
                  0.0,        0.0, 0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
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
            m_inv: self.m_inv * t.m_inv
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
        debug_assert!(wp != 0.0);

        let p = Point3f::new(xp, yp, zp);
        if wp == 1.0 {
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
