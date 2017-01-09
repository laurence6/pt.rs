// Type Float
macro_rules! DefFloat {
    ($f: ident) => (
        use std::$f;
        pub type Float = $f;
        pub const FLOAT_MAX: Float     = $f::MAX;
        pub const FLOAT_MIN_POS: Float = $f::MIN_POSITIVE;
        pub const EPSILON: Float       = $f::EPSILON * 0.5;
        pub const INFINITY: Float      = $f::INFINITY;

        // Max number less than 1
        pub const ONE_MINUS_EPSILON: Float = 1.0 - EPSILON;
    );
}

// Use f32 by default
#[cfg(not(feature = "f64"))]
DefFloat!(f32);
#[cfg(feature = "f64")]
DefFloat!(f64);

// Funcs
//pub fn Clamp<T: PartialOrd>(x: T, low: T, high: T) -> T {
//    //debug_assert!(low <= high);
//    if x < low {
//        return low;
//    } else if x > high {
//        return high;
//    } else {
//        return x;
//    }
// }
pub fn Clamp(x: Float, low: Float, high: Float) -> Float {
    debug_assert!(low <= high);
    if x < low {
        return low;
    } else if x > high {
        return high;
    } else {
        return x;
    }
}
