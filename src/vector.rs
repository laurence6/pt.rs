use std::ops;

use axis::Axis;
use common::Float;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f_new_and_ops!(Vector3f, x, y, z);
impl_vector3f_add!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_sub!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_index!(Vector3f);
impl_vector3f_from!(Point3f, Vector3f);
impl_vector3f_from!(Normal3f, Vector3f);

impl Vector3f {
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

    pub fn dot(&self, v: Vector3f) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Vector3f) -> Vector3f {
        Vector3f::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f_new_and_ops!(Normal3f, x, y, z);
impl_vector3f_from!(Vector3f, Normal3f);

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f_new_and_ops!(Point3f, x, y, z);
impl_vector3f_add!(Point3f, Point3f, Point3f, x, y, z);
impl_vector3f_add!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_sub!(Point3f, Point3f, Vector3f, x, y, z);
impl_vector3f_sub!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_index!(Point3f);
impl_vector3f_from!(Vector3f, Point3f);

impl Point3f {
    pub fn distance(&self, p: Point3f) -> Float {
        (*self - p).length()
    }

    pub fn min(&self, v: Point3f) -> Point3f {
        Point3f::new(
            self.x.min(v.x),
            self.y.min(v.y),
            self.z.min(v.z),
        )
    }

    pub fn max(&self, v: Point3f) -> Point3f {
        Point3f::new(
            self.x.max(v.x),
            self.y.max(v.y),
            self.z.max(v.z),
        )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Vector2, x, y);
impl_vector2_add!(Vector2, Vector2, Vector2, x, y);
impl_vector2_sub!(Vector2, Vector2, Vector2, x, y);
impl_vector2_index!(Vector2);

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Point2, x, y);
impl_vector2_add!(Point2, Point2, Point2, x, y);
impl_vector2_sub!(Point2, Point2, Vector2, x, y);
impl_vector2_index!(Point2);
impl_vector2_from!(Vector2, Point2);

pub type Point2u = Point2<u32>;
pub type Point2f = Point2<Float>;

impl From<Point2u> for Point2f {
    fn from(Point2u { x, y }: Point2u) -> Point2f {
        Point2f::new(
            x as Float,
            y as Float,
        )
    }
}
