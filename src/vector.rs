use std::ops;

use common::Float;
use axis::Axis;

macro_rules! impl_vector3f {
    ($name: ident, $x: ident, $y: ident, $z: ident) => (
        impl $name {
            pub fn new(x: Float, y: Float, z: Float) -> $name {
                $name { $x: x, $y: y, $z: z }
            }
        }

        impl ops::Neg for $name {
            type Output = $name;
            fn neg(self) -> $name {
                $name::new(
                    -self.$x,
                    -self.$y,
                    -self.$z,
                )
            }
        }

        impl ops::Add<Float> for $name {
            type Output = $name;
            fn add(self, a: Float) -> $name {
                $name::new(
                    self.$x + a,
                    self.$y + a,
                    self.$z + a,
                )
            }
        }

        impl ops::Sub<Float> for $name {
            type Output = $name;
            fn sub(self, a: Float) -> $name {
                $name::new(
                    self.$x - a,
                    self.$y - a,
                    self.$z - a,
                )
            }
        }

        impl ops::Mul<Float> for $name {
            type Output = $name;
            fn mul(self, a: Float) -> $name {
                $name::new(
                    self.$x * a,
                    self.$y * a,
                    self.$z * a,
                )
            }
        }

        impl ops::Div<Float> for $name {
            type Output = $name;
            fn div(self, a: Float) -> $name {
                $name::new(
                    self.$x / a,
                    self.$y / a,
                    self.$z / a,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_add {
    ($name: ident, $other: ident, $output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ops::Add<$other> for $name {
            type Output = $output;
            fn add(self, v: $other) -> $output {
                $output::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                    self.$z + v.$z,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_sub {
    ($name: ident, $other: ident, $output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ops::Sub<$other> for $name {
            type Output = $output;
            fn sub(self, v: $other) -> $output {
                $output::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                    self.$z - v.$z,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_index_axis {
    ($name: ident) => (
        impl ops::Index<Axis> for $name {
            type Output = Float;
            fn index(&self, axis: Axis) -> &Float {
                match axis {
                    Axis::X => &self.x,
                    Axis::Y => &self.y,
                    Axis::Z => &self.z,
                }
            }
        }

        impl ops::IndexMut<Axis> for $name {
            fn index_mut(&mut self, axis: Axis) -> &mut Float {
                match axis {
                    Axis::X => &mut self.x,
                    Axis::Y => &mut self.y,
                    Axis::Z => &mut self.z,
                }
            }
        }
    );
}

macro_rules! impl_vector3f_from {
    ($from: ident, $to: ident) => (
        impl From<$from> for $to {
            fn from(v: $from) -> $to {
                $to::new(v.x, v.y, v.z)
            }
        }
    );
}


#[derive(Default, Clone, Copy)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f!(Vector3f, x, y, z);
impl_vector3f_add!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_sub!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_index_axis!(Vector3f);
impl_vector3f_from!(Point3f, Vector3f);

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
}


#[derive(Default, Clone, Copy)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f!(Normal3f, x, y, z);


#[derive(Default, Clone, Copy)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl_vector3f!(Point3f, x, y, z);
impl_vector3f_add!(Point3f, Point3f, Point3f, x, y, z);
impl_vector3f_add!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_sub!(Point3f, Point3f, Vector3f, x, y, z);
impl_vector3f_sub!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_index_axis!(Point3f);
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


macro_rules! impl_vector2 {
    ($name: ident, $x: ident, $y: ident) => (
        impl<T> $name<T> where T: Copy {
            pub fn new(x: T, y: T) -> $name<T> {
                $name::<T>{ $x: x, $y: y }
            }
        }

        impl<T> ops::Add<T> for $name<T> where T: Copy + ops::Add<Output = T> {
            type Output = $name<T>;
            fn add(self, n: T) -> $name<T> {
                $name::<T>::new(
                    self.$x + n,
                    self.$y + n,
                )
            }
        }

        impl<T> ops::Sub<T> for $name<T> where T: Copy + ops::Sub<Output = T> {
            type Output = $name<T>;
            fn sub(self, n: T) -> $name<T> {
                $name::<T>::new(
                    self.$x - n,
                    self.$y - n,
                )
            }
        }
    );
}

macro_rules! impl_vector2_add {
    ($name: ident, $other: ident, $output: ident, $x: ident, $y: ident) => (
        impl<T> ops::Add<$other<T>> for $name<T> where T: Copy + ops::Add<Output = T> {
            type Output = $output<T>;
            fn add(self, v: $other<T>) -> $output<T> {
                $output::<T>::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                )
            }
        }
    );
}

macro_rules! impl_vector2_sub {
    ($name: ident, $other: ident, $output: ident, $x: ident, $y: ident) => (
        impl<T> ops::Sub<$other<T>> for $name<T> where T: Copy + ops::Sub<Output = T> {
            type Output = $output<T>;
            fn sub(self, v: $other<T>) -> $output<T> {
                $output::<T>::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                )
            }
        }
    );
}

macro_rules! impl_vector2_from {
    ($from: ident, $to: ident) => (
        impl<T> From<$from<T>> for $to<T> where T: Copy {
            fn from(v: $from<T>) -> $to<T> {
                $to::<T>::new(v.x, v.y)
            }
        }
    );
}


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2!(Vector2, x, y);
impl_vector2_add!(Vector2, Vector2, Vector2, x, y);
impl_vector2_sub!(Vector2, Vector2, Vector2, x, y);


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2!(Point2, x, y);
impl_vector2_add!(Point2, Point2, Point2, x, y);
impl_vector2_sub!(Point2, Point2, Vector2, x, y);
impl_vector2_from!(Vector2, Point2);

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
