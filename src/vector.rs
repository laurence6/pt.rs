use axis::Axis;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl_vector3f_new_and_ops!(Vector3f, x, y, z);
impl_vector3f_add!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_sub!(Vector3f, Vector3f, Vector3f, x, y, z);
impl_vector3f_index!(Vector3f, x, y, z);
impl_vector3f_index_axis!(Vector3f, x, y, z);
impl_vector3f_from!(Point3f, Vector3f);
impl_vector3f_from!(Normal3f, Vector3f);

impl Vector3f {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn abs(&self) -> Vector3f {
        Vector3f::new(
            self.x.abs(),
            self.y.abs(),
            self.z.abs(),
        )
    }

    pub fn normalize(&self) -> Vector3f {
        let l = self.length();
        Vector3f::new(
            self.x / l,
            self.y / l,
            self.z / l,
        )
    }

    pub fn dot(&self, v: Vector3f) -> f32 {
          self.x * v.x
        + self.y * v.y
        + self.z * v.z
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
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl_vector3f_new_and_ops!(Normal3f, x, y, z);
impl_vector3f_from!(Vector3f, Normal3f);

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Point3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl_vector3f_new_and_ops!(Point3f, x, y, z);
impl_vector3f_add!(Point3f, Point3f, Point3f, x, y, z);
impl_vector3f_add!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_sub!(Point3f, Point3f, Vector3f, x, y, z);
impl_vector3f_sub!(Point3f, Vector3f, Point3f, x, y, z);
impl_vector3f_index!(Point3f, x, y, z);
impl_vector3f_index_axis!(Point3f, x, y, z);
impl_vector3f_from!(Vector3f, Point3f);

impl Point3f {
    pub fn distance(&self, p: Point3f) -> f32 {
        (*self - p).length()
    }

    pub fn min(&self, p: Point3f) -> Point3f {
        Point3f::new(
            self.x.min(p.x),
            self.y.min(p.y),
            self.z.min(p.z),
        )
    }

    pub fn max(&self, p: Point3f) -> Point3f {
        Point3f::new(
            self.x.max(p.x),
            self.y.max(p.y),
            self.z.max(p.z),
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
pub struct Point2<T> where T: PartialOrd {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Point2, x, y);
impl_vector2_add!(Point2, Point2, Point2, x, y);
impl_vector2_sub!(Point2, Point2, Vector2, x, y);
impl_vector2_index!(Point2);
impl_vector2_from!(Vector2, Point2);

pub type Point2u = Point2<u32>;
pub type Point2f = Point2<f32>;

impl<T> Point2<T> where T: Copy + PartialOrd {
    pub fn min(&self, p: Point2<T>) -> Point2<T> {
        Point2::new(
            min(self.x, p.x),
            min(self.y, p.y),
        )
    }

    pub fn max(&self, p: Point2<T>) -> Point2<T> {
        Point2::new(
            max(self.x, p.x),
            max(self.y, p.y),
        )
    }
}

impl From<Point2u> for Point2f {
    fn from(Point2u { x, y }: Point2u) -> Point2f {
        Point2f::new(
            x as f32,
            y as f32,
        )
    }
}

fn min<T>(v1: T, v2: T) -> T where T: PartialOrd {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

fn max<T>(v1: T, v2: T) -> T where T: PartialOrd {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
