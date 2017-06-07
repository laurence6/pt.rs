use std::ops;
use std::fmt::{Debug, Formatter, Error};

use common::Float;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Matrix {
    m: [[Float; 4]; 4],
}

impl From<[[Float; 4]; 4]> for Matrix {
    fn from(m: [[Float; 4]; 4]) -> Matrix {
        Matrix { m: m }
    }
}

impl Matrix {
    pub fn new(
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

    pub fn transpose(&self) -> Matrix {
        Matrix::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    pub fn inverse(&self) -> Matrix {
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
        debug_assert!(d != 0.);

        r = r / d;

        return r;
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

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Matrix {{\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n}}",
            self[0][0], self[0][1], self[0][2], self[0][3],
            self[1][0], self[1][1], self[1][2], self[1][3],
            self[2][0], self[2][1], self[2][2], self[2][3],
            self[3][0], self[3][1], self[3][2], self[3][3],
        )
    }
}

#[cfg(test)]
mod matrix_test {
    use matrix::Matrix;

    #[test]
    fn test_inverse() {
        let m = Matrix::new(
            100.,     0., 0., 200.,
              0.,  -100., 0., 100.,
              0.,     0., 1.,   0.,
              0.,     0., 0.,   1.,
        );
        let invm = Matrix::new(
            0.01,   0., 0., -2.,
             0., -0.01, 0.,  1.,
             0.,    0., 1.,  0.,
             0.,    0., 0.,  1.,
        );
        assert_eq!(m.inverse(), invm);
    }
}
