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

macro_rules! impl_vector2_new_and_ops {
    ($vector2: ident, $x: ident, $y: ident) => (
        impl<T> $vector2<T> where T: Copy {
            pub fn new(x: T, y: T) -> $vector2<T> {
                $vector2::<T> { $x: x, $y: y }
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
            fn from($vector2_from::<T> { x, y }: $vector2_from<T>) -> $vector2_to<T> {
                $vector2_to::<T>::new(x, y)
            }
        }
    );
}
