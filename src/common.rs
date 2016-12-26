// Type Float
macro_rules! DefFloat {
    ($v: ident) => (
        use std::$v;
        pub type Float = $v;
        pub const FLOAT_MAX: Float = $v::MAX;
        pub const FLOAT_MIN_POS: Float = $v::MIN_POSITIVE;
    );
}

DefFloat!(f32);

// Const
pub const INF: Float = 2e9;
pub const EPS: Float = 1e-9;

// Axis
use std::convert::Into;
#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn NextAxis(&self) -> Axis {
        match *self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
    pub fn OtherAxes(&self) -> (Axis, Axis) {
        match *self {
            Axis::X => (Axis::Y, Axis::Z),
            Axis::Y => (Axis::X, Axis::Z),
            Axis::Z => (Axis::X, Axis::Y),
        }
    }
}

impl Into<usize> for Axis {
    fn into(self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

// funcs
pub fn clamp(x: Float) -> Float {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}
