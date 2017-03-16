use std::ops;

use common::Float;
use axis::Axis;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vector3f {
    pub X: Float,
    pub Y: Float,
    pub Z: Float,
}

fn V(x: Float, y: Float, z: Float) -> Vector3f {
    Vector3f::New(x, y, z)
}

impl Vector3f {
    pub fn New(x: Float, y: Float, z: Float) -> Vector3f {
        Vector3f { X: x, Y: y, Z: z }
    }

    fn Length(&self) -> Float {
        (self.X * self.X + self.Y * self.Y + self.Z * self.Z).sqrt()
    }

    pub fn Normalize(&self) -> Vector3f {
        let l = self.Length();
        V(self.X / l, self.Y / l, self.Z / l)
    }

    pub fn Inv(&self) -> Vector3f {
        V(1 as Float / self.X, 1 as Float / self.Y, 1 as Float / self.Z)
    }

    fn Abs(&self) -> Vector3f {
        V(self.X.abs(), self.Y.abs(), self.Z.abs())
    }

    pub fn Dot(&self, v: Vector3f) -> Float {
        self.X * v.X + self.Y * v.Y + self.Z * v.Z
    }

    pub fn Cross(&self, v: Vector3f) -> Vector3f {
        let x = self.Y * v.Z - self.Z * v.Y;
        let y = self.Z * v.X - self.X * v.Z;
        let z = self.X * v.Y - self.Y * v.X;
        return V(x, y, z);
    }

    pub fn Min(&self, v: Vector3f) -> Vector3f {
        V(self.X.min(v.X), self.Y.min(v.Y), self.Z.min(v.Z))
    }

    pub fn Max(&self, v: Vector3f) -> Vector3f {
        V(self.X.max(v.X), self.Y.max(v.Y), self.Z.max(v.Z))
    }

    fn MinAxis(&self) -> Vector3f {
        let (x, y, z) = (self.X.abs(), self.Y.abs(), self.Z.abs());
        match (x <= y, y <= z) {
             (true,  true) => return V(1.0, 0.0, 0.0),
             (false, true) => return V(0.0, 1.0, 0.0),
             _             => return V(0.0, 0.0, 1.0),
        }
    }

    fn MinComponent(&self) -> Float {
        self.X.min(self.Y).min(self.Z)
    }

    fn MaxComponent(&self) -> Float {
        self.X.max(self.Y).max(self.Z)
    }
}

impl ops::Neg for Vector3f {
    type Output = Vector3f;
    fn neg(self) -> Vector3f {
        V(-self.X, -self.Y, -self.Z)
    }
}

impl ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn add(self, v: Vector3f) -> Vector3f {
        V(self.X + v.X, self.Y + v.Y, self.Z + v.Z)
    }
}

impl ops::Sub<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn sub(self, v: Vector3f) -> Vector3f {
        V(self.X - v.X, self.Y - v.Y, self.Z - v.Z)
    }
}

impl ops::Add<Float> for Vector3f {
    type Output = Vector3f;
    fn add(self, a: Float) -> Vector3f {
        V(self.X + a, self.Y + a, self.Z + a)
    }
}

impl ops::Sub<Float> for Vector3f {
    type Output = Vector3f;
    fn sub(self, a: Float) -> Vector3f {
        V(self.X - a, self.Y - a, self.Z - a)
    }
}

impl ops::Mul<Float> for Vector3f {
    type Output = Vector3f;
    fn mul(self, a: Float) -> Vector3f {
        V(self.X * a, self.Y * a, self.Z * a)
    }
}

impl ops::Div<Float> for Vector3f {
    type Output = Vector3f;
    fn div(self, a: Float) -> Vector3f {
        V(self.X / a, self.Y / a, self.Z / a)
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
    fn index_mut(&mut self, axis: Axis) -> &mut Float {
        match axis {
            Axis::X => &mut self.X,
            Axis::Y => &mut self.Y,
            Axis::Z => &mut self.Z,
        }
    }
}

pub type Vector = Vector3f;
pub type Point3f = Vector3f;

impl Point3f {
    pub fn Distance(self, p: Point3f) -> Float {
        (self - p).Length()
    }
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Vector2<T> where T: Copy {
    pub X: T,
    pub Y: T,
}

impl<T> Vector2<T> where T: Copy {
    pub fn New(x: T, y: T) -> Vector2<T> {
        Vector2::<T>{ X: x, Y: y }
    }
}

impl<T> ops::Add<T> for Vector2<T> where T: Copy + ops::Add<Output = T> {
    type Output = Vector2<T>;
    fn add(self, n: T) -> Vector2<T> {
        Vector2::<T>{
            X: self.X + n,
            Y: self.Y + n,
        }
    }
}

impl<T> ops::Add<Vector2<T>> for Vector2<T> where T: Copy + ops::Add<Output = T> {
    type Output = Vector2<T>;
    fn add(self, p: Vector2<T>) -> Vector2<T> {
        Vector2::<T>{
            X: self.X + p.X,
            Y: self.Y + p.Y,
        }
    }
}

pub type Point2u = Vector2<u64>;
pub type Point2f = Vector2<Float>;

impl Point2f {
    pub fn From(p: Point2u) -> Point2f {
        Point2f {
            X: p.X as Float,
            Y: p.Y as Float,
        }
    }
}
