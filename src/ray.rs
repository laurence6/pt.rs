use common::Float;
use vector::Vector;

#[derive(Debug)]
pub struct Ray {
    pub Origin: Vector,
    pub Direction: Vector,
}


impl Ray {
    pub fn New(origin: Vector, direction: Vector) -> Ray {
        return Ray { Origin: origin, Direction: direction };
    }

    pub fn Position(&self, t: Float) -> Vector {
        return self.Origin.Add(&self.Direction.MulScalar(t));
    }

    pub fn Reflect(&self, i: &Ray) -> Ray {
        return Ray::New(self.Origin, self.Direction.Reflect(&i.Direction));
    }

    pub fn Refract(&self, i: &Ray, n1: Float, n2: Float) -> Ray {
        return Ray::New(self.Origin, self.Direction.Refract(&i.Direction, n1, n2));
    }

    pub fn Reflectance(&self, i: &Ray, n1: Float, n2: Float) -> Float {
        return self.Direction.Reflectance(&i.Direction, n1, n2);
    }

    pub fn WeightedBounce() -> Ray {
        unimplemented!()
    }
}
