use std::ops;

use common::Float;
use vector::{Vector3f, Point3f};
use ray::Ray;
use bbox::BBox3f;

const IDENTITY_MATRIX: Matrix = Matrix { m: [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
] };

#[derive(Default, Clone, Copy, PartialEq)]
struct Matrix {
    m: [[Float; 4]; 4],
}

fn m4(
    m00: Float, m01: Float, m02: Float, m03: Float,
    m10: Float, m11: Float, m12: Float, m13: Float,
    m20: Float, m21: Float, m22: Float, m23: Float,
    m30: Float, m31: Float, m32: Float, m33: Float,
) -> Matrix {
    Matrix { m: [
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
        [m30, m31, m32, m33],
    ] }
}

impl Matrix {
    pub fn new(m: [[Float; 4]; 4]) -> Matrix {
        Matrix { m: m }
    }

    fn transpose(&self) -> Matrix {
        m4(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    fn inverse(&self) -> Matrix {
        let mut r = Matrix::default();

        r[0][0] = self[1][2] * self[2][3] * self[3][1] - self[1][3] * self[2][2] * self[3][1] + self[1][3] * self[2][1] * self[3][2] - self[1][1] * self[2][3] * self[3][2] - self[1][2] * self[2][1] * self[3][3] + self[1][1] * self[2][2] * self[3][3];
        r[0][1] = self[0][3] * self[2][2] * self[3][1] - self[0][2] * self[2][3] * self[3][1] - self[0][3] * self[2][1] * self[3][2] + self[0][1] * self[2][3] * self[3][2] + self[0][2] * self[2][1] * self[3][3] - self[0][1] * self[2][2] * self[3][3];
        r[0][2] = self[0][2] * self[1][3] * self[3][1] - self[0][3] * self[1][2] * self[3][1] + self[0][3] * self[1][1] * self[3][2] - self[0][1] * self[1][3] * self[3][2] - self[0][2] * self[1][1] * self[3][3] + self[0][1] * self[1][2] * self[3][3];
        r[0][3] = self[0][3] * self[1][2] * self[2][1] - self[0][2] * self[1][3] * self[2][1] - self[0][3] * self[1][1] * self[2][2] + self[0][1] * self[1][3] * self[2][2] + self[0][2] * self[1][1] * self[2][3] - self[0][1] * self[1][2] * self[2][3];
        r[1][0] = self[1][3] * self[2][2] * self[3][0] - self[1][2] * self[2][3] * self[3][0] - self[1][3] * self[2][0] * self[3][2] + self[1][0] * self[2][3] * self[3][2] + self[1][2] * self[2][0] * self[3][3] - self[1][0] * self[2][2] * self[3][3];
        r[1][1] = self[0][2] * self[2][3] * self[3][0] - self[0][3] * self[2][2] * self[3][0] + self[0][3] * self[2][0] * self[3][2] - self[0][0] * self[2][3] * self[3][2] - self[0][2] * self[2][0] * self[3][3] + self[0][0] * self[2][2] * self[3][3];
        r[1][2] = self[0][3] * self[1][2] * self[3][0] - self[0][2] * self[1][3] * self[3][0] - self[0][3] * self[1][0] * self[3][2] + self[0][0] * self[1][3] * self[3][2] + self[0][2] * self[1][0] * self[3][3] - self[0][0] * self[1][2] * self[3][3];
        r[1][3] = self[0][2] * self[1][3] * self[2][0] - self[0][3] * self[1][2] * self[2][0] + self[0][3] * self[1][0] * self[2][2] - self[0][0] * self[1][3] * self[2][2] - self[0][2] * self[1][0] * self[2][3] + self[0][0] * self[1][2] * self[2][3];
        r[2][0] = self[1][1] * self[2][3] * self[3][0] - self[1][3] * self[2][1] * self[3][0] + self[1][3] * self[2][0] * self[3][1] - self[1][0] * self[2][3] * self[3][1] - self[1][1] * self[2][0] * self[3][3] + self[1][0] * self[2][1] * self[3][3];
        r[2][1] = self[0][3] * self[2][1] * self[3][0] - self[0][1] * self[2][3] * self[3][0] - self[0][3] * self[2][0] * self[3][1] + self[0][0] * self[2][3] * self[3][1] + self[0][1] * self[2][0] * self[3][3] - self[0][0] * self[2][1] * self[3][3];
        r[2][2] = self[0][1] * self[1][3] * self[3][0] - self[0][3] * self[1][1] * self[3][0] + self[0][3] * self[1][0] * self[3][1] - self[0][0] * self[1][3] * self[3][1] - self[0][1] * self[1][0] * self[3][3] + self[0][0] * self[1][1] * self[3][3];
        r[2][3] = self[0][3] * self[1][1] * self[2][0] - self[0][1] * self[1][3] * self[2][0] - self[0][3] * self[1][0] * self[2][1] + self[0][0] * self[1][3] * self[2][1] + self[0][1] * self[1][0] * self[2][3] - self[0][0] * self[1][1] * self[2][3];
        r[3][0] = self[1][2] * self[2][1] * self[3][0] - self[1][1] * self[2][2] * self[3][0] - self[1][2] * self[2][0] * self[3][1] + self[1][0] * self[2][2] * self[3][1] + self[1][1] * self[2][0] * self[3][2] - self[1][0] * self[2][1] * self[3][2];
        r[3][1] = self[0][1] * self[2][2] * self[3][0] - self[0][2] * self[2][1] * self[3][0] + self[0][2] * self[2][0] * self[3][1] - self[0][0] * self[2][2] * self[3][1] - self[0][1] * self[2][0] * self[3][2] + self[0][0] * self[2][1] * self[3][2];
        r[3][2] = self[0][2] * self[1][1] * self[3][0] - self[0][1] * self[1][2] * self[3][0] - self[0][2] * self[1][0] * self[3][1] + self[0][0] * self[1][2] * self[3][1] + self[0][1] * self[1][0] * self[3][2] - self[0][0] * self[1][1] * self[3][2];
        r[3][3] = self[0][1] * self[1][2] * self[2][0] - self[0][2] * self[1][1] * self[2][0] + self[0][2] * self[1][0] * self[2][1] - self[0][0] * self[1][2] * self[2][1] - self[0][1] * self[1][0] * self[2][2] + self[0][0] * self[1][1] * self[2][2];

        let d =
              self[0][3] * self[1][2] * self[2][1] * self[3][0] - self[0][2] * self[1][3] * self[2][1] * self[3][0] - self[0][3] * self[1][1] * self[2][2] * self[3][0] + self[0][1] * self[1][3] * self[2][2] * self[3][0]
            + self[0][2] * self[1][1] * self[2][3] * self[3][0] - self[0][1] * self[1][2] * self[2][3] * self[3][0] - self[0][3] * self[1][2] * self[2][0] * self[3][1] + self[0][2] * self[1][3] * self[2][0] * self[3][1]
            + self[0][3] * self[1][0] * self[2][2] * self[3][1] - self[0][0] * self[1][3] * self[2][2] * self[3][1] - self[0][2] * self[1][0] * self[2][3] * self[3][1] + self[0][0] * self[1][2] * self[2][3] * self[3][1]
            + self[0][3] * self[1][1] * self[2][0] * self[3][2] - self[0][1] * self[1][3] * self[2][0] * self[3][2] - self[0][3] * self[1][0] * self[2][1] * self[3][2] + self[0][0] * self[1][3] * self[2][1] * self[3][2]
            + self[0][1] * self[1][0] * self[2][3] * self[3][2] - self[0][0] * self[1][1] * self[2][3] * self[3][2] - self[0][2] * self[1][1] * self[2][0] * self[3][3] + self[0][1] * self[1][2] * self[2][0] * self[3][3]
            + self[0][2] * self[1][0] * self[2][1] * self[3][3] - self[0][0] * self[1][2] * self[2][1] * self[3][3] - self[0][1] * self[1][0] * self[2][2] * self[3][3] + self[0][0] * self[1][1] * self[2][2] * self[3][3];
        debug_assert!(d != 0.0);

        r = r / d;

        return r;
    }

    fn apply_point(&self, p: Point3f) -> Point3f {
        let xp = self[0][0] * p.x + self[0][1] * p.y + self[0][2] * p.z + self[0][3];
        let yp = self[1][0] * p.x + self[1][1] * p.y + self[1][2] * p.z + self[1][3];
        let zp = self[2][0] * p.x + self[2][1] * p.y + self[2][2] * p.z + self[2][3];
        let wp = self[3][0] * p.x + self[3][1] * p.y + self[3][2] * p.z + self[3][3];
        debug_assert!(wp != 0.0);

        let p = Point3f::new(xp, yp, zp);
        if wp == 1.0 {
            return p;
        } else {
            return p / wp;
        }
    }

    fn apply_vector(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self[0][0] * v.x + self[0][1] * v.y + self[0][2] * v.z,
            self[1][0] * v.x + self[1][1] * v.y + self[1][2] * v.z,
            self[2][0] * v.x + self[2][1] * v.y + self[2][2] * v.z,
        )
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, m: Matrix) -> Matrix {
        let mut r = Matrix::default();
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] =
                    self[i][0] * m[0][j] +
                    self[i][1] * m[1][j] +
                    self[i][2] * m[2][j] +
                    self[i][3] * m[3][j];
            }
        }
        return r;
    }
}

impl ops::Div<Float> for Matrix {
    type Output = Matrix;
    fn div(self, n: Float) -> Matrix {
        let mut r = Matrix::default();
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self[i][j] / n;
            }
        }
        return r;
    }
}

impl ops::Index<usize> for Matrix {
    type Output = [Float; 4];
    fn index(&self, i: usize) -> &[Float; 4] {
        &self.m[i]
    }
}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut [Float; 4] {
        &mut self.m[i]
    }
}

#[derive(Clone, Copy)]
pub struct Transform {
    m: Matrix,
    m_inv: Matrix,
}

impl Transform {
    fn from_single_mat(m: [[Float; 4]; 4]) -> Transform {
        let mat = Matrix::new(m);
        return Transform {
            m: mat,
            m_inv: mat.inverse(),
        };
    }

    fn from_mats(m: [[Float; 4]; 4], m_inv: [[Float; 4]; 4]) -> Transform {
        Transform { m: Matrix::new(m), m_inv: Matrix::new(m_inv) }
    }

    pub fn translate(v: Vector3f) -> Transform {
        Transform {
            m: m4(
                1.0, 0.0, 0.0, v.x,
                0.0, 1.0, 0.0, v.y,
                0.0, 0.0, 1.0, v.z,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: m4(
                1.0, 0.0, 0.0, -v.x,
                0.0, 1.0, 0.0, -v.y,
                0.0, 0.0, 1.0, -v.z,
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn scale(v: Vector3f) -> Transform {
        Transform {
            m: m4(
                v.x, 0.0, 0.0, 0.0,
                0.0, v.y, 0.0, 0.0,
                0.0, 0.0, v.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            m_inv: m4(
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
        let p = m4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, f / (f - n), - f * n / (f - n),
            0.0, 0.0, 1.0, 0.0,
        );
        let inv_tan_ang = 1.0 / (fov.to_radians() / 2.0).tan();
        return Transform::scale(Vector3f::new(inv_tan_ang, inv_tan_ang, 1.0))
             * Transform::from_single_mat(p.m);
    }

    /// Compute look-at transformation from camera position, a point camera looks at and up
    /// direction in world space coordinates.
    fn look_at(pos: Vector3f, look: Vector3f, up: Vector3f) -> Transform {
        let d = (look - pos).normalize();
        let up = up.normalize();
        let left = up.cross(d).normalize();
        let up = d.cross(left);

        let camera_to_world = m4(
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
        let m = m4(
            1.0,       0.0,        0.0, 0.0,
            0.0, cos_theta, -sin_theta, 0.0,
            0.0, sin_theta,  cos_theta, 0.0,
            0.0,       0.0,        0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
    }

    fn rotate_y(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = m4(
             cos_theta, 0.0, sin_theta, 0.0,
                  0.0,  1.0,       0.0, 0.0,
            -sin_theta, 0.0, cos_theta, 0.0,
                  0.0,  0.0,       0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
    }

    fn rotate_z(&self, theta: Float) -> Transform {
        let (sin_theta, cos_theta) = compute_sin_cos_in_degree(theta);
        let m = m4(
            cos_theta, -sin_theta, 0.0, 0.0,
            sin_theta,  cos_theta, 0.0, 0.0,
                  0.0,        0.0, 1.0, 0.0,
                  0.0,        0.0, 0.0, 1.0,
        );
        return Transform { m: m, m_inv: m.transpose() };
    }

    pub fn apply_point(&self, p: Point3f) -> Point3f {
        self.m.apply_point(p)
    }

    fn apply_vector(&self, v: Vector3f) -> Vector3f {
        self.m.apply_vector(v)
    }

    fn apply_normal(&self, n: Vector3f) -> Vector3f {
        self.m_inv.apply_vector(n)
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
