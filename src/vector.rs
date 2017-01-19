use std::ops;
use std::convert;

use common::Float;
use axis::Axis;

pub type Vector = Vector3f;
pub const ZERO_VECTOR: Vector3f = Vector3f { X: 0.0, Y: 0.0, Z: 0.0 };

#[derive(Clone, Copy, Debug)]
pub struct Vector3f {
    pub X: Float,
    pub Y: Float,
    pub Z: Float,
}

fn V(x: Float, y: Float, z: Float) -> Vector3f {
    return Vector3f::New(x, y, z);
}

impl Vector3f {
    pub fn New(x: Float, y: Float, z: Float) -> Vector3f {
        return Vector3f { X: x, Y: y, Z: z };
    }

    pub fn Length(&self) -> Float {
        return (self.X * self.X + self.Y * self.Y + self.Z * self.Z).sqrt();
    }

    pub fn Normalize(&self) -> Vector3f {
        let l = self.Length();
        return V(self.X / l, self.Y / l, self.Z / l);
    }

    pub fn Inv(&self) -> Vector3f {
        return V(1.0 / self.X, 1.0 / self.Y, 1.0 / self.Z);
    }

    pub fn Abs(&self) -> Vector3f {
        return V(self.X.abs(), self.Y.abs(), self.Z.abs());
    }

    pub fn Dot(&self, v: &Vector3f) -> Float {
        return self.X * v.X + self.Y * v.Y + self.Z * v.Z;
    }

    pub fn Cross(&self, v: &Vector3f) -> Vector3f {
        let x = self.Y * v.Z - self.Z * v.Y;
        let y = self.Z * v.X - self.X * v.Z;
        let z = self.X * v.Y - self.Y * v.X;
        return V(x, y, z);
    }

    pub fn Min(&self, v: &Vector3f) -> Vector3f {
        return V(self.X.min(v.X), self.Y.min(v.Y), self.Z.min(v.Z));
    }

    pub fn Max(&self, v: &Vector3f) -> Vector3f {
        return V(self.X.max(v.X), self.Y.max(v.Y), self.Z.max(v.Z));
    }

    pub fn MinAxis(&self) -> Vector3f {
        let (x, y, z) = (self.X.abs(), self.Y.abs(), self.Z.abs());
        match (x <= y, y <= z) {
             (true,  true) => return V(1.0, 0.0, 0.0),
             (false, true) => return V(0.0, 1.0, 0.0),
             _             => return V(0.0, 0.0, 1.0),
        }
    }

    pub fn MinComponent(&self) -> Float {
        return self.X.min(self.Y).min(self.Z);
    }

    pub fn MaxComponent(&self) -> Float {
        return self.X.max(self.Y).max(self.Z);
    }
}

impl ops::Neg for Vector3f {
    type Output = Vector3f;
    fn neg(self) -> Vector3f {
        return V(-self.X, -self.Y, -self.Z);
    }
}

impl ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn add(self, v: Vector3f) -> Vector3f {
        return V(self.X + v.X, self.Y + v.Y, self.Z + v.Z);
    }
}

impl ops::Sub<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn sub(self, v: Vector3f) -> Vector3f {
        return V(self.X - v.X, self.Y - v.Y, self.Z - v.Z);
    }
}

impl ops::Add<Float> for Vector3f {
    type Output = Vector3f;
    fn add(self, a: Float) -> Vector3f {
        return V(self.X + a, self.Y + a, self.Z + a);
    }
}

impl ops::Sub<Float> for Vector3f {
    type Output = Vector3f;
    fn sub(self, a: Float) -> Vector3f {
        return V(self.X - a, self.Y - a, self.Z - a);
    }
}

impl ops::Mul<Float> for Vector3f {
    type Output = Vector3f;
    fn mul(self, a: Float) -> Vector3f {
        return V(self.X * a, self.Y * a, self.Z * a);
    }
}

impl ops::Div<Float> for Vector3f {
    type Output = Vector3f;
    fn div(self, a: Float) -> Vector3f {
        return V(self.X / a, self.Y / a, self.Z / a);
    }
}

impl ops::Index<Axis> for Vector3f {
    type Output = Float;
    fn index(&self, axis: Axis) -> &Float {
        match axis {
            Axis::X => &self.X,
            Axis::Y => &self.Y,
            Axis::Z => &self.Z,
        }
    }
}

impl ops::IndexMut<Axis> for Vector3f {
    fn index_mut<'a>(&'a mut self, axis: Axis) -> &'a mut Float {
        match axis {
            Axis::X => &mut self.X,
            Axis::Y => &mut self.Y,
            Axis::Z => &mut self.Z,
        }
    }
}

macro_rules! point2 {
    ($n: ident, $t: ident) => (
        #[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub X: $t,
            pub Y: $t,
        }

        impl $n {
            pub fn New(x: $t, y: $t) -> $n {
                return $n { X: x, Y: y };
            }
        }

        impl ops::Add<$t> for $n {
            type Output = $n;
            fn add(self, n: $t) -> $n {
                return $n {
                    X: self.X + n,
                    Y: self.Y + n,
                }
            }
        }

        impl ops::Add<$n> for $n {
            type Output = $n;
            fn add(self, p: $n) -> $n {
                return $n {
                    X: self.X + p.X,
                    Y: self.Y + p.Y,
                }
            }
        }
    )
}

point2!(Point2i, i32);
point2!(Point2f, Float);

//pub type Point2f = Point2<Float>;
//pub type Point2i = Point2<i32>;
//
//#[derive(Clone, Copy, Debug)]
//pub struct Point2<T> {
//    pub X: T,
//    pub Y: T,
// }
//
//impl<T> Point2<T> {
//    pub fn New(x: T, y: T) -> Point2<T> {
//        return Point2{ X: x, Y: y };
//    }
// }
//
//impl<T: ops::Add<Output = T>> ops::Add<Point2<T>> for Point2<T> {
//    type Output = Point2<T>;
//    fn add(self, p: Point2<T>) -> Point2<T> {
//        return Point2::<T>{
//            X: self.X + p.X,
//            Y: self.Y + p.Y,
//        }
//    }
// }
//
//impl<T: ops::Add<Output = T> + Copy> ops::Add<T> for Point2<T> {
//    type Output = Point2<T>;
//    fn add(self, n: T) -> Point2<T> {
//        return Point2::<T>{
//            X: self.X + n,
//            Y: self.Y + n,
//        }
//    }
// }

impl Point2f {
    pub fn From(p: Point2i) -> Point2f {
        return Point2f {
            X: p.X as f32,
            Y: p.Y as f32,
        }
    }
}
