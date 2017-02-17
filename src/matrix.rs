use common::Float;
use vector::Vector;

pub struct Matrix {
    pub m11: Float, pub m12: Float, pub m13: Float, pub m14: Float,
    pub m21: Float, pub m22: Float, pub m23: Float, pub m24: Float,
    pub m31: Float, pub m32: Float, pub m33: Float, pub m34: Float,
    pub m41: Float, pub m42: Float, pub m43: Float, pub m44: Float,
}

pub const IDENTITY_MATRIX: Matrix = Matrix {
    m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
    m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
    m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
    m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
};

pub fn Translate(v: Vector) -> Matrix {
    return Matrix {
        m11: 1.0, m12: 0.0, m13: 0.0, m14: v.X,
        m21: 0.0, m22: 1.0, m23: 0.0, m24: v.Y,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: v.Z,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
    };
}
