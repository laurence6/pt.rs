// Type Float
macro_rules! DefFloat {
    ($v: ident) => (
        use std::$v;
        pub type Float = $v;
        pub const FLOAT_MAX: Float      = $v::MAX;
        pub const FLOAT_MIN_POS: Float  = $v::MIN_POSITIVE;
        pub const EPSILON: Float        = $v::EPSILON;
        pub const EPSILON_HALF: Float   = EPSILON * 0.5;
        pub const INFINITY: Float       = $v::INFINITY;
    );
}

DefFloat!(f32);

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
pub fn gamma(x: Float) -> Float {
    return (x * EPSILON_HALF) / (1.0 - x * EPSILON_HALF);
}

pub fn clamp(x: Float) -> Float {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}
