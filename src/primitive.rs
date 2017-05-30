use shape::Shape;
use material::Material;

pub trait Primitive : Shape + Material {}

impl<T> Primitive for T where T: Shape + Material {}
