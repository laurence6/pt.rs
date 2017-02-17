use std::ops;

use common::Float;
use axis::Axis;

pub const ZERO_VECTOR: Vector = Vector { X: 0.0, Y: 0.0, Z: 0.0 };

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub X: Float,
    pub Y: Float,
    pub Z: Float,
}

fn V(x: Float, y: Float, z: Float) -> Vector {
    Vector::New(x, y, z)
}

impl Vector {
    pub fn New(x: Float, y: Float, z: Float) -> Vector {
        Vector { X: x, Y: y, Z: z }
    }

    pub fn Length(&self) -> Float {
        (self.X * self.X + self.Y * self.Y + self.Z * self.Z).sqrt()
    }

    pub fn Normalize(&self) -> Vector {
        let l = self.Length();
        V(self.X / l, self.Y / l, self.Z / l)
    }

    pub fn Inv(&self) -> Vector {
        V(1.0 / self.X, 1.0 / self.Y, 1.0 / self.Z)
    }

    pub fn Abs(&self) -> Vector {
        V(self.X.abs(), self.Y.abs(), self.Z.abs())
    }

    pub fn Dot(&self, v: Vector) -> Float {
        self.X * v.X + self.Y * v.Y + self.Z * v.Z
    }

    pub fn Cross(&self, v: Vector) -> Vector {
        let x = self.Y * v.Z - self.Z * v.Y;
        let y = self.Z * v.X - self.X * v.Z;
        let z = self.X * v.Y - self.Y * v.X;
        return V(x, y, z);
    }

    pub fn Min(&self, v: Vector) -> Vector {
        V(self.X.min(v.X), self.Y.min(v.Y), self.Z.min(v.Z))
    }

    pub fn Max(&self, v: Vector) -> Vector {
        V(self.X.max(v.X), self.Y.max(v.Y), self.Z.max(v.Z))
    }

    pub fn MinAxis(&self) -> Vector {
        let (x, y, z) = (self.X.abs(), self.Y.abs(), self.Z.abs());
        match (x <= y, y <= z) {
             (true,  true) => return V(1.0, 0.0, 0.0),
             (false, true) => return V(0.0, 1.0, 0.0),
             _             => return V(0.0, 0.0, 1.0),
        }
    }

    pub fn MinComponent(&self) -> Float {
        self.X.min(self.Y).min(self.Z)
    }

    pub fn MaxComponent(&self) -> Float {
        self.X.max(self.Y).max(self.Z)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        V(-self.X, -self.Y, -self.Z)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        V(self.X + v.X, self.Y + v.Y, self.Z + v.Z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector {
        V(self.X - v.X, self.Y - v.Y, self.Z - v.Z)
    }
}

impl ops::Add<Float> for Vector {
    type Output = Vector;
    fn add(self, a: Float) -> Vector {
        V(self.X + a, self.Y + a, self.Z + a)
    }
}

impl ops::Sub<Float> for Vector {
    type Output = Vector;
    fn sub(self, a: Float) -> Vector {
        V(self.X - a, self.Y - a, self.Z - a)
    }
}

impl ops::Mul<Float> for Vector {
    type Output = Vector;
    fn mul(self, a: Float) -> Vector {
        V(self.X * a, self.Y * a, self.Z * a)
    }
}

impl ops::Div<Float> for Vector {
    type Output = Vector;
    fn div(self, a: Float) -> Vector {
        V(self.X / a, self.Y / a, self.Z / a)
    }
}

impl ops::Index<Axis> for Vector {
    type Output = Float;
    fn index(&self, axis: Axis) -> &Float {
        match axis {
            Axis::X => &self.X,
            Axis::Y => &self.Y,
            Axis::Z => &self.Z,
        }
    }
}

impl ops::IndexMut<Axis> for Vector {
    fn index_mut(&mut self, axis: Axis) -> &mut Float {
        match axis {
            Axis::X => &mut self.X,
            Axis::Y => &mut self.Y,
            Axis::Z => &mut self.Z,
        }
    }
}

macro_rules! point2 {
    ($n: ident, $t: ident) => (
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $n {
            pub X: $t,
            pub Y: $t,
        }

        impl $n {
            pub fn New(x: $t, y: $t) -> $n {
                $n { X: x, Y: y }
            }
        }

        impl ops::Add<$t> for $n {
            type Output = $n;
            fn add(self, n: $t) -> $n {
                $n {
                    X: self.X + n,
                    Y: self.Y + n,
                }
            }
        }

        impl ops::Add<$n> for $n {
            type Output = $n;
            fn add(self, p: $n) -> $n {
                $n {
                    X: self.X + p.X,
                    Y: self.Y + p.Y,
                }
            }
        }
    )
}

point2!(Point2i, u64);
point2!(Point2f, Float);

impl Point2f {
    pub fn From(p: Point2i) -> Point2f {
        Point2f {
            X: p.X as Float,
            Y: p.Y as Float,
        }
    }
}
