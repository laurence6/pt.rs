macro_rules! defFloat {
    ($t: ident) => (
        pub type Float = $t;

        use std::$t;
        pub const FLOAT_MAX: Float     = $t::MAX;
        pub const FLOAT_MIN_POS: Float = $t::MIN_POSITIVE;
        pub const EPSILON: Float       = $t::EPSILON * 0.5;
        pub const INFINITY: Float      = $t::INFINITY;
        /// Max number less than 1.
        pub const ONE_MINUS_EPSILON: Float = 1. - EPSILON;

        pub const PI: Float = $t::consts::PI;
    );
}

// Use f32 by default
#[cfg(not(feature = "f64"))]
defFloat!(f32);
#[cfg(feature = "f64")]
defFloat!(f64);

// Funcs
pub fn clamp<T: PartialOrd>(x: T, low: T, high: T) -> T {
    debug_assert!(low <= high);
    if x < low {
        return low;
    } else if x > high {
        return high;
    } else {
        return x;
    }
}

pub fn lerp(t: Float, v1: Float, v2: Float) -> Float {
    (1. - t) * v1 + t * v2
}
