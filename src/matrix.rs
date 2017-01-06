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

pub fn Translate(v: &Vector) -> Matrix {
    return Matrix {
        m11: 1.0, m12: 0.0, m13: 0.0, m14: v.X,
        m21: 0.0, m22: 1.0, m23: 0.0, m24: v.Y,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: v.Z,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
    };
}
//
//pub fn Scale(v: &Vector) -> Matrix {
//    return Matrix {
//        m11: v.X, m12: 0.0, m13: 0.0, m14: 0.0,
//        m21: 0.0, m22: v.Y, m23: 0.0, m24: 0.0,
//        m31: 0.0, m32: 0.0, m33: v.Z, m34: 0.0,
//        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
//    };
// }
//
//pub fn Rotate(v: &Vector, a: Float) -> Matrix {
//    let v = v.Normalize();
//    let s = a.sin();
//    let c = a.cos();
//    let m = 1.0 - c;
//    return Matrix {
//        m11: m*v.X*v.X + c,     m12: m*v.X*v.Y + v.Z*s, m13: m*v.Z*v.X - v.Y*s, m14: 0.0,
//        m21: m*v.X*v.Y - v.Z*s, m22: m*v.Y*v.Y + c,     m23: m*v.Y*v.Z + v.X*s, m24: 0.0,
//        m31: m*v.Z*v.X + v.Y*s, m32: m*v.Y*v.Z - v.X*s, m33: m*v.Z*v.Z + c,     m34: 0.0,
//        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
//    }
// }
