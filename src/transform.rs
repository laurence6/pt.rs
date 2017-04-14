use std::ops;

use common::Float;
use vector::{Vector3f, Normal3f, Point3f};
use ray::Ray;
use bbox::BBox3f;
use matrix::Matrix;

#[derive(Clone, Copy)]
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
    pub fn translate(v: Vector3f) -> Transform {
        Transform {
            m: Matrix::new(
                1.0, 0.0, 0.0, v.x,
                0.0, 1.0, 0.0, v.y,
                0.0, 0.0, 1.0, v.z,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix::new(
                1.0, 0.0, 0.0, -v.x,
                0.0, 1.0, 0.0, -v.y,
                0.0, 0.0, 1.0, -v.z,
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn scale(v: Vector3f) -> Transform {
        Transform {
            m: Matrix::new(
                v.x, 0.0, 0.0, 0.0,
                0.0, v.y, 0.0, 0.0,
                0.0, 0.0, v.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: Matrix::new(
                1.0/v.x, 0.0,     0.0,     0.0,
                0.0,     1.0/v.y, 0.0,     0.0,
                0.0,     0.0,     1.0/v.z, 0.0,
                0.0,     0.0,     0.0,     1.0,
            ),
        }
    }

    /// Compute perspective transformation from field-of-view angel, distance to near a near z
    /// plane and a far z plane.
    pub fn perspective(fov: Float, n: Float, f: Float) -> Transform {
        let p = Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, f / (f - n), - f * n / (f - n),
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

    pub fn apply_point(&self, p: Point3f) -> Point3f {
        let xp = self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z + self.m[0][3];
        let yp = self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z + self.m[1][3];
        let zp = self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z + self.m[2][3];
        let wp = self.m[3][0] * p.x + self.m[3][1] * p.y + self.m[3][2] * p.z + self.m[3][3];
        debug_assert!(wp != 0.0);

        let p = Point3f::new(xp, yp, zp);
        if wp == 1.0 {
            return p;
        } else {
            return p / wp;
        }
    }

    pub fn apply_vector(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }

    pub fn apply_normal(&self, n: Normal3f) -> Normal3f {
        Normal3f::new(
            self.m_inv[0][0] * n.x + self.m_inv[1][0] * n.y + self.m_inv[2][0] * n.z,
            self.m_inv[0][1] * n.x + self.m_inv[1][1] * n.y + self.m_inv[2][1] * n.z,
            self.m_inv[0][2] * n.x + self.m_inv[1][2] * n.y + self.m_inv[2][2] * n.z,
        )
    }

    pub fn apply_ray(&self, r: &Ray) -> Ray {
        unimplemented!()
    }

    fn apply_bbox(&self, b: &BBox3f) -> BBox3f {
        unimplemented!()
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
