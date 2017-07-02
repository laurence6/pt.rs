macro_rules! impl_vector3f_new_and_ops {
    ($vector3f: ident, $x: ident, $y: ident, $z: ident) => (
        impl $vector3f {
            pub fn new($x: Float, $y: Float, $z: Float) -> $vector3f {
                $vector3f { $x, $y, $z }
            }
        }

        impl ::std::ops::Neg for $vector3f {
            type Output = $vector3f;
            fn neg(self) -> $vector3f {
                $vector3f::new(
                    -self.$x,
                    -self.$y,
                    -self.$z,
                )
            }
        }

        impl ::std::ops::Add<Float> for $vector3f {
            type Output = $vector3f;
            fn add(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x + a,
                    self.$y + a,
                    self.$z + a,
                )
            }
        }

        impl ::std::ops::AddAssign<Float> for $vector3f {
            fn add_assign(&mut self, a: Float) {
                self.$x += a;
                self.$y += a;
                self.$z += a;
            }
        }

        impl ::std::ops::Sub<Float> for $vector3f {
            type Output = $vector3f;
            fn sub(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x - a,
                    self.$y - a,
                    self.$z - a,
                )
            }
        }

        impl ::std::ops::SubAssign<Float> for $vector3f {
            fn sub_assign(&mut self, a: Float) {
                self.$x -= a;
                self.$y -= a;
                self.$z -= a;
            }
        }

        impl ::std::ops::Mul<Float> for $vector3f {
            type Output = $vector3f;
            fn mul(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x * a,
                    self.$y * a,
                    self.$z * a,
                )
            }
        }

        impl ::std::ops::MulAssign<Float> for $vector3f {
            fn mul_assign(&mut self, a: Float) {
                self.$x *= a;
                self.$y *= a;
                self.$z *= a;
            }
        }

        impl ::std::ops::Div<Float> for $vector3f {
            type Output = $vector3f;
            fn div(self, a: Float) -> $vector3f {
                $vector3f::new(
                    self.$x / a,
                    self.$y / a,
                    self.$z / a,
                )
            }
        }

        impl ::std::ops::DivAssign<Float> for $vector3f {
            fn div_assign(&mut self, a: Float) {
                self.$x /= a;
                self.$y /= a;
                self.$z /= a;
            }
        }
    );
}

macro_rules! impl_vector3f_add {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ::std::ops::Add<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn add(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                    self.$z + v.$z,
                )
            }
        }

        impl ::std::ops::AddAssign<$vector3f_other> for $vector3f {
            fn add_assign(&mut self, v: $vector3f_other) {
                self.$x += v.$x;
                self.$y += v.$y;
                self.$z += v.$z;
            }
        }
    );
}

macro_rules! impl_vector3f_sub {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ::std::ops::Sub<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn sub(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                    self.$z - v.$z,
                )
            }
        }

        impl ::std::ops::SubAssign<$vector3f_other> for $vector3f {
            fn sub_assign(&mut self, v: $vector3f_other) {
                self.$x -= v.$x;
                self.$y -= v.$y;
                self.$z -= v.$z;
            }
        }
    );
}

macro_rules! impl_vector3f_mul {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ::std::ops::Mul<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn mul(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x * v.$x,
                    self.$y * v.$y,
                    self.$z * v.$z,
                )
            }
        }

        impl ::std::ops::MulAssign<$vector3f_other> for $vector3f {
            fn mul_assign(&mut self, v: $vector3f_other) {
                self.$x *= v.$x;
                self.$y *= v.$y;
                self.$z *= v.$z;
            }
        }
    );
}

macro_rules! impl_vector3f_div {
    ($vector3f: ident, $vector3f_other: ident, $vector3f_output: ident, $x: ident, $y: ident, $z: ident) => (
        impl ::std::ops::Div<$vector3f_other> for $vector3f {
            type Output = $vector3f_output;
            fn div(self, v: $vector3f_other) -> $vector3f_output {
                $vector3f_output::new(
                    self.$x / v.$x,
                    self.$y / v.$y,
                    self.$z / v.$z,
                )
            }
        }

        impl ::std::ops::DivAssign<$vector3f_other> for $vector3f {
            fn div_assign(&mut self, v: $vector3f_other) {
                self.$x /= v.$x;
                self.$y /= v.$y;
                self.$z /= v.$z;
            }
        }
    );
}

macro_rules! impl_vector3f_index {
    ($vector3f: ident) => (
        impl ::std::ops::Index<usize> for $vector3f {
            type Output = Float;
            fn index(&self, i: usize) -> &Float {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!(),
                }
            }
        }

        impl ::std::ops::IndexMut<usize> for $vector3f {
            fn index_mut(&mut self, i: usize) -> &mut Float {
                match i {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!(),
                }
            }
        }

        impl ::std::ops::Index<Axis> for $vector3f {
            type Output = Float;
            fn index(&self, axis: Axis) -> &Float {
                match axis {
                    Axis::X => &self.x,
                    Axis::Y => &self.y,
                    Axis::Z => &self.z,
                }
            }
        }

        impl ::std::ops::IndexMut<Axis> for $vector3f {
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

macro_rules! impl_vector2_new_and_ops {
    ($vector2: ident, $x: ident, $y: ident) => (
        impl<T> $vector2<T> {
            pub fn new($x: T, $y: T) -> $vector2<T> {
                $vector2::<T> { $x, $y }
            }
        }

        impl<T> ::std::ops::Neg for $vector2<T> where T: Copy + ::std::ops::Neg<Output = T> {
            type Output = $vector2<T>;
            fn neg(self) -> $vector2<T> {
                $vector2::<T>::new(
                    -self.$x,
                    -self.$y,
                )
            }
        }

        impl<T> ::std::ops::Add<T> for $vector2<T> where T: Copy + ::std::ops::Add<Output = T> {
            type Output = $vector2<T>;
            fn add(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x + n,
                    self.$y + n,
                )
            }
        }

        impl<T> ::std::ops::AddAssign<T> for $vector2<T> where T: Copy + ::std::ops::AddAssign<T> {
            fn add_assign(&mut self, n: T) {
                self.$x += n;
                self.$y += n;
            }
        }

        impl<T> ::std::ops::Sub<T> for $vector2<T> where T: Copy + ::std::ops::Sub<Output = T> {
            type Output = $vector2<T>;
            fn sub(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x - n,
                    self.$y - n,
                )
            }
        }

        impl<T> ::std::ops::SubAssign<T> for $vector2<T> where T: Copy + ::std::ops::SubAssign<T> {
            fn sub_assign(&mut self, n: T) {
                self.$x -= n;
                self.$y -= n;
            }
        }

        impl<T> ::std::ops::Mul<T> for $vector2<T> where T: Copy + ::std::ops::Mul<Output = T> {
            type Output = $vector2<T>;
            fn mul(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x * n,
                    self.$y * n,
                )
            }
        }

        impl<T> ::std::ops::MulAssign<T> for $vector2<T> where T: Copy + ::std::ops::MulAssign<T> {
            fn mul_assign(&mut self, n: T) {
                self.$x *= n;
                self.$y *= n;
            }
        }

        impl<T> ::std::ops::Div<T> for $vector2<T> where T: Copy + ::std::ops::Div<Output = T> {
            type Output = $vector2<T>;
            fn div(self, n: T) -> $vector2<T> {
                $vector2::<T>::new(
                    self.$x / n,
                    self.$y / n,
                )
            }
        }

        impl<T> ::std::ops::DivAssign<T> for $vector2<T> where T: Copy + ::std::ops::DivAssign<T> {
            fn div_assign(&mut self, n: T) {
                self.$x /= n;
                self.$y /= n;
            }
        }
    );
}

macro_rules! impl_vector2_add {
    ($vector2: ident, $vector2_other: ident, $vector2_output: ident, $x: ident, $y: ident) => (
        impl<T> ::std::ops::Add<$vector2_other<T>> for $vector2<T> where T: Copy + ::std::ops::Add<Output = T> {
            type Output = $vector2_output<T>;
            fn add(self, v: $vector2_other<T>) -> $vector2_output<T> {
                $vector2_output::<T>::new(
                    self.$x + v.$x,
                    self.$y + v.$y,
                )
            }
        }

        impl<T> ::std::ops::AddAssign<$vector2_other<T>> for $vector2<T> where T: Copy + ::std::ops::AddAssign<T> {
            fn add_assign(&mut self, v: $vector2_other<T>) {
                self.$x += v.$x;
                self.$y += v.$y;
            }
        }
    );
}

macro_rules! impl_vector2_sub {
    ($vector2: ident, $vector2_other: ident, $vector2_output: ident, $x: ident, $y: ident) => (
        impl<T> ::std::ops::Sub<$vector2_other<T>> for $vector2<T> where T: Copy + ::std::ops::Sub<Output = T> {
            type Output = $vector2_output<T>;
            fn sub(self, v: $vector2_other<T>) -> $vector2_output<T> {
                $vector2_output::<T>::new(
                    self.$x - v.$x,
                    self.$y - v.$y,
                )
            }
        }

        impl<T> ::std::ops::SubAssign<$vector2_other<T>> for $vector2<T> where T: Copy + ::std::ops::SubAssign<T> {
            fn sub_assign(&mut self, v: $vector2_other<T>) {
                self.$x -= v.$x;
                self.$y -= v.$y;
            }
        }
    );
}

macro_rules! impl_vector2_index {
    ($vector2: ident) => (
        impl<T> ::std::ops::Index<usize> for $vector2<T> {
            type Output = T;
            fn index(&self, i: usize) -> &T {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!(),
                }
            }
        }

        impl<T> ::std::ops::IndexMut<usize> for $vector2<T> {
            fn index_mut(&mut self, i: usize) -> &mut T {
                match i {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!(),
                }
            }
        }
    );
}

macro_rules! impl_vector2_from {
    ($vector2_from: ident, $vector2_to: ident) => (
        impl<T> From<$vector2_from<T>> for $vector2_to<T> where T: Copy {
            fn from($vector2_from::<T> { x, y }: $vector2_from<T>) -> $vector2_to<T> {
                $vector2_to::<T>::new(x, y)
            }
        }
    );
}
