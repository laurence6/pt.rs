// Type Float
macro_rules! DefFloat {
    ($v: ident) => (
        use std::$v;
        pub type Float = $v;
        pub const FLOAT_MAX: Float     = $v::MAX;
        pub const FLOAT_MIN_POS: Float = $v::MIN_POSITIVE;
        pub const EPSILON: Float       = $v::EPSILON * 0.5;
        pub const INFINITY: Float      = $v::INFINITY;

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
pub fn clamp(x: Float) -> Float {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}
