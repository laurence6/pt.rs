use std::ops;

use common::Float;
use axis::Axis;

#[derive(Default, Clone, Copy)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3f {
    pub fn new(x: Float, y: Float, z: Float) -> Vector3f {
        Vector3f { x: x, y: y, z: z }
    }

    pub fn length(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3f {
        let l = self.length();
        Vector3f::new(
            self.x / l,
            self.y / l,
            self.z / l,
        )
    }

    pub fn inv(&self) -> Vector3f {
        Vector3f::new(
            1 as Float / self.x,
            1 as Float / self.y,
            1 as Float / self.z,
        )
    }

    pub fn dot(&self, v: Vector3f) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Vector3f) -> Vector3f {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        return Vector3f::new(x, y, z);
    }

    pub fn min(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x.min(v.x),
            self.y.min(v.y),
            self.z.min(v.z),
        )
    }

    pub fn max(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x.max(v.x),
            self.y.max(v.y),
            self.z.max(v.z),
        )
    }
}

impl ops::Neg for Vector3f {
    type Output = Vector3f;
    fn neg(self) -> Vector3f {
        Vector3f::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

impl ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn add(self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z,
        )
    }
}

impl ops::Sub<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn sub(self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.x - v.x,
            self.y - v.y,
            self.z - v.z,
        )
    }
}

impl ops::Add<Float> for Vector3f {
    type Output = Vector3f;
    fn add(self, a: Float) -> Vector3f {
        Vector3f::new(
            self.x + a,
            self.y + a,
            self.z + a,
        )
    }
}

impl ops::Sub<Float> for Vector3f {
    type Output = Vector3f;
    fn sub(self, a: Float) -> Vector3f {
        Vector3f::new(
            self.x - a,
            self.y - a,
            self.z - a,
        )
    }
}

impl ops::Mul<Float> for Vector3f {
    type Output = Vector3f;
    fn mul(self, a: Float) -> Vector3f {
        Vector3f::new(
            self.x * a,
            self.y * a,
            self.z * a,
        )
    }
}

impl ops::Div<Float> for Vector3f {
    type Output = Vector3f;
    fn div(self, a: Float) -> Vector3f {
        Vector3f::new(
            self.x / a,
            self.y / a,
            self.z / a,
        )
    }
}

impl ops::Index<Axis> for Vector3f {
    type Output = Float;
    fn index(&self, axis: Axis) -> &Float {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl ops::IndexMut<Axis> for Vector3f {
    fn index_mut(&mut self, axis: Axis) -> &mut Float {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

pub type Point3f = Vector3f;

impl Point3f {
    pub fn distance(self, p: Point3f) -> Float {
        (self - p).length()
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Vector2<T> where T: Copy {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> where T: Copy {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2::<T>{ x: x, y: y }
    }
}

impl<T> ops::Add<T> for Vector2<T> where T: Copy + ops::Add<Output = T> {
    type Output = Vector2<T>;
    fn add(self, n: T) -> Vector2<T> {
        Vector2::<T>::new(
            self.x + n,
            self.y + n,
        )
    }
}

impl<T> ops::Sub<T> for Vector2<T> where T: Copy + ops::Sub<Output = T> {
    type Output = Vector2<T>;
    fn sub(self, n: T) -> Vector2<T> {
        Vector2::<T>::new(
            self.x - n,
            self.y - n,
        )
    }
}

impl<T> ops::Add<Vector2<T>> for Vector2<T> where T: Copy + ops::Add<Output = T> {
    type Output = Vector2<T>;
    fn add(self, v: Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.x + v.x,
            self.y + v.y,
        )
    }
}

impl<T> ops::Sub<Vector2<T>> for Vector2<T> where T: Copy + ops::Sub<Output = T> {
    type Output = Vector2<T>;
    fn sub(self, v: Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.x - v.x,
            self.y - v.y,
        )
    }
}

pub type Point2<T> = Vector2<T>;
pub type Point2u = Point2<u32>;
pub type Point2f = Point2<Float>;

impl From<Point2u> for Point2f {
    fn from(p: Point2u) -> Point2f {
        Point2f::new(
            p.x as Float,
            p.y as Float,
        )
    }
}
