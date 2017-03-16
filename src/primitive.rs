use shape::Shape;
use material::Material;

pub trait Primitive : Shape + Material {}
