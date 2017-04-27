use std::ops;

use common::Float;
use axis::Axis;

macro_rules! impl_vector3f_new_and_ops {
    ($vector3f: ident, $x: ident, $y: ident, $z: ident) => (
        impl $vector3f {
            pub fn new(x: Float, y: Float, z: Float) -> $vector3f {
                $vector3f { $x: x, $y: y, $z: z }
            }
        }

        impl ops::Neg for $vector3f {
            type Output = $vector3f;
            fn neg(self) -> $vector3f {
                $vector3f::new(
                    -self.$x,
                    -self.$y,
                    -self.$z,
                )
            }
        }

        impl ops::Add<Float> for $vector3f {
            type Output = $vector3f;
            fn add(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x + a,
                    self.$y + a,
                    self.$z + a,
                )
            }
        }

        impl ops::Sub<Float> for $vector3f {
            type Output = $vector3f;
            fn sub(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x - a,
                    self.$y - a,
                    self.$z - a,
                )
            }
        }

        impl ops::Mul<Float> for $vector3f {
            type Output = $vector3f;
            fn mul(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x * a,
                    self.$y * a,
                    self.$z * a,
                )
            }
        }

        impl ops::Div<Float> for $vector3f {
            type Output = $vector3f;
            fn div(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x / a,
                    self.$y / a,
                    self.$z / a,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_add {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ops::Add<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn add(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                    self.$z + v.$z,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_sub {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ops::Sub<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn sub(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                    self.$z - v.$z,
                )
            }
        }
    );
}

macro_rules! impl_vector3f_index_axis {
    ($vector3f: ident) => (
        impl ops::Index<Axis> for $vector3f {
            type Output = Float;
            fn index(&self, axis: Axis) -> &Float {
                match axis {
                    Axis::X => &self.x,
                    Axis::Y => &self.y,
                    Axis::Z => &self.z,
                }
            }
        }

        impl ops::IndexMut<Axis> for $vector3f {
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
    ($vector3f_from: ident, $vector3f_to: ident) => (
        impl From<$vector3f_from> for $vector3f_to {
            fn from($vector3f_from { x, y, z }: $vector3f_from) -> $vector3f_to {
                $vector3f_to::new(x, y, z)
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
impl_vector3f_new_and_ops!(Vector3f, x, y, z);
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
impl_vector3f_new_and_ops!(Normal3f, x, y, z);


#[derive(Default, Clone, Copy)]
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


macro_rules! impl_vector2_new_and_ops {
    ($vector2: ident, $x: ident, $y: ident) => (
        impl<T> $vector2<T> where T: Copy {
            pub fn new(x: T, y: T) -> $vector2<T> {
                $vector2::<T>{ $x: x, $y: y }
            }
        }

        impl<T> ops::Add<T> for $vector2<T> where T: Copy + ops::Add<Output = T> {
            type Output = $vector2<T>;
            fn add(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x + n,
                    self.$y + n,
                )
            }
        }

        impl<T> ops::Sub<T> for $vector2<T> where T: Copy + ops::Sub<Output = T> {
            type Output = $vector2<T>;
            fn sub(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x - n,
                    self.$y - n,
                )
            }
        }
    );
}

macro_rules! impl_vector2_add {
    ($vector2: ident, $vector2_other: ident, $vector2_output: ident, $x: ident, $y: ident) => (
        impl<T> ops::Add<$vector2_other<T>> for $vector2<T> where T: Copy + ops::Add<Output = T> {
            type Output = $vector2_output<T>;
            fn add(self, v: $vector2_other<T>) -> $vector2_output<T> {
                $vector2_output::<T>::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                )
            }
        }
    );
}

macro_rules! impl_vector2_sub {
    ($vector2: ident, $vector2_other: ident, $vector2_output: ident, $x: ident, $y: ident) => (
        impl<T> ops::Sub<$vector2_other<T>> for $vector2<T> where T: Copy + ops::Sub<Output = T> {
            type Output = $vector2_output<T>;
            fn sub(self, v: $vector2_other<T>) -> $vector2_output<T> {
                $vector2_output::<T>::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                )
            }
        }
    );
}

macro_rules! impl_vector2_from {
    ($vector2_from: ident, $vector2_to: ident) => (
        impl<T> From<$vector2_from<T>> for $vector2_to<T> where T: Copy {
            fn from($vector2_from::<T>{ x, y }: $vector2_from<T>) -> $vector2_to<T> {
                $vector2_to::<T>::new(x, y)
            }
        }
    );
}


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Vector2, x, y);
impl_vector2_add!(Vector2, Vector2, Vector2, x, y);
impl_vector2_sub!(Vector2, Vector2, Vector2, x, y);


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Point2, x, y);
impl_vector2_add!(Point2, Point2, Point2, x, y);
impl_vector2_sub!(Point2, Point2, Vector2, x, y);
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
