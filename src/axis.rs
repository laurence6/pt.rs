use std::convert;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    /// Return next axis.
    pub fn NextAxis(&self) -> Axis {
        match *self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }

    /// Return other two axes.
    pub fn OtherAxes(&self) -> (Axis, Axis) {
        match *self {
            Axis::X => (Axis::Y, Axis::Z),
            Axis::Y => (Axis::X, Axis::Z),
            Axis::Z => (Axis::X, Axis::Y),
        }
    }
}

impl convert::Into<usize> for Axis {
    /// Convert axis to usize
    ///
    /// X -> 0
    ///
    /// Y -> 1
    ///
    /// Z -> 3
    fn into(self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}
