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
    pub fn from_spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector3f {
        Vector3f::new(
            sin_theta * phi.cos(),
            sin_theta * phi.sin(),
            cos_theta,
        )
    }

    pub fn max_abs_axis(&self) -> Axis {
        let Vector3f { x, y, z } = self.abs();
        match (x >= y, x >= z, y >= z) {
            ( true,  true,     _) => Axis::X,
            (false,     _,  true) => Axis::Y,
            (    _, false, false) => Axis::Z,
            _                     => panic!(),
        }
    }

    pub fn max_component(&self) -> f32 {
        (self.x).max(self.y).max(self.z)
    }

    pub fn permute(&self, x: Axis, y: Axis, z: Axis) -> Vector3f {
        Vector3f::new(
            self[x],
            self[y],
            self[z],
        )
    }

    /// Construct a coordinate system based on a normalized vector.
    pub fn construct_coordinate_system(&self) -> (Vector3f, Vector3f) {
        let v1 = if self.x.abs() > self.y.abs() {
            Vector3f::new(-self.z, 0., self.x) / (self.x * self.x + self.z * self.z).sqrt()
        } else {
            Vector3f::new(0., self.z, -self.y) / (self.y * self.y + self.z * self.z).sqrt()
        };
        let v2 = self.cross(v1);
        return (v1, v2);
    }

    pub fn dot(&self, v: Vector3f) -> f32 {
          self.x * v.x
        + self.y * v.y
        + self.z * v.z
    }

    pub fn dot_n(&self, n: Normal3f) -> f32 {
          self.x * n.x
        + self.y * n.y
        + self.z * n.z
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
impl_vector3f_add!(Normal3f, Normal3f, Normal3f, x, y, z);
impl_vector3f_from!(Vector3f, Normal3f);
impl_vector3f_from!(Point3f, Normal3f);

impl Normal3f {
    pub fn dot(&self, n: Normal3f) -> f32 {
          self.x * n.x
        + self.y * n.y
        + self.z * n.z
    }
}

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
    pub fn permute(&self, x: Axis, y: Axis, z: Axis) -> Point3f {
        Point3f::new(
            self[x],
            self[y],
            self[z],
        )
    }

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

pub type Vector2u = Vector2<u32>;
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
impl_vector2_new_and_ops!(Point2, x, y);
impl_vector2_add!(Point2, Point2, Point2, x, y);
impl_vector2_add!(Point2, Vector2, Point2, x, y);
impl_vector2_sub!(Point2, Point2, Vector2, x, y);
impl_vector2_sub!(Point2, Vector2, Point2, x, y);
impl_vector2_index!(Point2);
impl_vector2_from!(Vector2, Point2);

pub type Point2u = Point2<u32>;
pub type Point2i = Point2<i32>;
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

impl Point2f {
    pub fn floor(&self) -> Point2f {
        Point2f::new(
            self.x.floor(),
            self.y.floor(),
        )
    }

    pub fn ceil(&self) -> Point2f {
        Point2f::new(
            self.x.ceil(),
            self.y.ceil(),
        )
    }
}

impl From<Point2i> for Point2u {
    fn from(Point2 { x, y }: Point2i) -> Point2u {
        debug_assert!(x >= 0, "x = {} is less than 0", x);
        debug_assert!(y >= 0, "y = {} is less than 0", y);

        Point2::new(
            x as u32,
            y as u32,
        )
    }
}

impl From<Point2f> for Point2u {
    fn from(Point2 { x, y }: Point2f) -> Point2u {
        debug_assert!(x >= 0., "x = {} is less than 0", x);
        debug_assert!(y >= 0., "y = {} is less than 0", y);

        Point2::new(
            x as u32,
            y as u32,
        )
    }
}

impl From<Point2u> for Point2i {
    fn from(Point2 { x, y }: Point2u) -> Point2i {
        Point2::new(
            x as i32,
            y as i32,
        )
    }
}

impl From<Point2f> for Point2i {
    fn from(Point2 { x, y }: Point2f) -> Point2i {
        Point2::new(
            x as i32,
            y as i32,
        )
    }
}

impl From<Point2u> for Point2f {
    fn from(Point2 { x, y }: Point2u) -> Point2f {
        Point2::new(
            x as f32,
            y as f32,
        )
    }
}

impl From<Point2i> for Point2f {
    fn from(Point2 { x, y }: Point2i) -> Point2f {
        Point2::new(
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
