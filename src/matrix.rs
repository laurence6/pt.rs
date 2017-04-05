use std::ops;

use common::Float;
use vector::Vector3f;
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

fn M(
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
    pub fn New(m: [[Float; 4]; 4]) -> Matrix {
        Matrix { m: m }
    }

    fn Transpose(&self) -> Matrix {
        M(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    fn Inverse(&self) -> Matrix {
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

    fn ApplyPoint(&self, p: Vector3f) -> Vector3f {
        let xp = self[0][0] * p.X + self[0][1] * p.Y + self[0][2] * p.Z + self[0][3];
        let yp = self[1][0] * p.X + self[1][1] * p.Y + self[1][2] * p.Z + self[1][3];
        let zp = self[2][0] * p.X + self[2][1] * p.Y + self[2][2] * p.Z + self[2][3];
        let wp = self[3][0] * p.X + self[3][1] * p.Y + self[3][2] * p.Z + self[3][3];
        debug_assert!(wp != 0.0);

        let p = Vector3f::New(xp, yp, zp);
        if wp == 1.0 {
            return p;
        } else {
            return p / wp;
        }
    }

    fn ApplyVector(&self, v: Vector3f) -> Vector3f {
        Vector3f::New(
            self.m[0][0] * v.X + self.m[0][1] * v.Y + self.m[0][2] * v.Z,
            self.m[1][0] * v.X + self.m[1][1] * v.Y + self.m[1][2] * v.Z,
            self.m[2][0] * v.X + self.m[2][1] * v.Y + self.m[2][2] * v.Z,
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
    mInv: Matrix,
}

impl Transform {
    fn FromSingleMat(m: [[Float; 4]; 4]) -> Transform {
        let mat = Matrix::New(m);
        return Transform {
            m: mat,
            mInv: mat.Inverse(),
        };
    }

    fn FromMats(m: [[Float; 4]; 4], mInv: [[Float; 4]; 4]) -> Transform {
        Transform { m: Matrix::New(m), mInv: Matrix::New(mInv) }
    }

    pub fn Translate(v: Vector3f) -> Transform {
        Transform {
            m: M(
                1.0, 0.0, 0.0, v.X,
                0.0, 1.0, 0.0, v.Y,
                0.0, 0.0, 1.0, v.Z,
                0.0, 0.0, 0.0, 1.0,
            ),
            mInv: M(
                1.0, 0.0, 0.0, -v.X,
                0.0, 1.0, 0.0, -v.Y,
                0.0, 0.0, 1.0, -v.Z,
                0.0, 0.0, 0.0, 1.0,
            ),
        }
    }

    pub fn Scale(v: Vector3f) -> Transform {
        Transform {
            m: M(
                v.X, 0.0, 0.0, 0.0,
                0.0, v.Y, 0.0, 0.0,
                0.0, 0.0, v.Z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ),
            mInv: M(
                1.0/v.X, 0.0,     0.0,     0.0,
                0.0,     1.0/v.Y, 0.0,     0.0,
                0.0,     0.0,     1.0/v.Z, 0.0,
                0.0,     0.0,     0.0,     1.0,
            ),
        }
    }

    /// Compute perspective transformation from field-of-view angel, distance to near a near z
    /// plane and a far z plane.
    pub fn Perspective(fov: Float, n: Float, f: Float) -> Transform {
        let p = M(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, f / (f - n), - f * n / (f - n),
            0.0, 0.0, 1.0, 0.0,
        );
        let invTanAng = 1.0 / (fov.to_radians() / 2.0).tan();
        return Transform::Scale(Vector3f::New(invTanAng, invTanAng, 1.0))
            * Transform::FromSingleMat(p.m);
    }

    /// Compute look-at transformation from camera position, a point camera looks at and up
    /// direction in world space coordinates.
    fn LookAt(pos: Vector3f, look: Vector3f, up: Vector3f) -> Transform {
        let d = (look - pos).Normalize();
        let up = up.Normalize();
        let left = up.Cross(d).Normalize();
        let up = d.Cross(left);

        let camera_to_world = M(
            left.X, up.X, d.X, pos.X,
            left.Y, up.Y, d.Y, pos.Y,
            left.Z, up.Z, d.Z, pos.Z,
               0.0,  0.0, 0.0,   1.0,
        );

        return Transform { m: camera_to_world.Inverse(), mInv: camera_to_world };
    }

    fn Transpose(&self) -> Transform {
        Transform {
            m: self.m.Transpose(),
            mInv: self.mInv.Transpose(),
        }
    }

    pub fn Inverse(&self) -> Transform {
        Transform { m: self.mInv, mInv: self.m }
    }

    fn RotateX(&self, theta: Float) -> Transform {
        let (sinTheta, cosTheta) = computeSinCosInDegree(theta);
        let m = M(
            1.0,      0.0,       0.0, 0.0,
            0.0, cosTheta, -sinTheta, 0.0,
            0.0, sinTheta,  cosTheta, 0.0,
            0.0,      0.0,       0.0, 1.0,
        );
        return Transform { m: m, mInv: m.Transpose() };
    }

    fn RotateY(&self, theta: Float) -> Transform {
        let (sinTheta, cosTheta) = computeSinCosInDegree(theta);
        let m = M(
             cosTheta, 0.0, sinTheta, 0.0,
                  0.0, 1.0,      0.0, 0.0,
            -sinTheta, 0.0, cosTheta, 0.0,
                  0.0, 0.0,      0.0, 1.0,
        );
        return Transform { m: m, mInv: m.Transpose() };
    }

    fn RotateZ(&self, theta: Float) -> Transform {
        let (sinTheta, cosTheta) = computeSinCosInDegree(theta);
        let m = M(
            cosTheta, -sinTheta, 0.0, 0.0,
            sinTheta,  cosTheta, 0.0, 0.0,
                 0.0,       0.0, 1.0, 0.0,
                 0.0,       0.0, 0.0, 1.0,
        );
        return Transform { m: m, mInv: m.Transpose() };
    }

    pub fn ApplyPoint(&self, p: Vector3f) -> Vector3f {
        self.m.ApplyPoint(p)
    }

    fn ApplyVector(&self, v: Vector3f) -> Vector3f {
        self.m.ApplyVector(v)
    }

    fn ApplyNormal(&self, n: Vector3f) -> Vector3f {
        self.mInv.ApplyVector(n)
    }

    pub fn ApplyRay(&self, r: &Ray) -> Ray {
        unimplemented!()
    }

    fn ApplyBBox(&self, b: &BBox3f) -> BBox3f {
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
            mInv: self.mInv * t.mInv
        }
    }
}

fn computeSinCosInDegree(deg: Float) -> (Float, Float) {
    (deg.to_radians().sin(), deg.to_radians().cos())
}
