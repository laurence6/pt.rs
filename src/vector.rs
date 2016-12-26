use std::ops;

extern crate rand;

use common::Float;
use common::Axis;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub X: Float,
    pub Y: Float,
    pub Z: Float,
}

pub fn V(x: Float, y: Float, z: Float) -> Vector {
    return Vector::New(x, y, z);
}

// TODO: Dont use reference. Use Copy.

impl Vector {
    pub fn New(x: Float, y: Float, z: Float) -> Vector {
        return Vector { X: x, Y: y, Z: z };
    }

    pub fn ZeroVector() -> Vector {
        return V(0.0, 0.0, 0.0);
    }

    pub fn RandomUnitVector() -> Vector {
        let x = rand::random::<Float>();
        let y = rand::random::<Float>();
        let z = rand::random::<Float>();
        return V(x, y, z).Normalize();
    }

    pub fn Length(&self) -> Float {
        return (self.X * self.X + self.Y * self.Y + self.Z * self.Z).sqrt();
    }

    pub fn Normalize(&self) -> Vector {
        let l = self.Length();
        return V(self.X / l, self.Y / l, self.Z / l);
    }

    pub fn Neg(&self) -> Vector {
        return V(-self.X, -self.Y, -self.Z);
    }

    pub fn Abs(&self) -> Vector {
        return V(self.X.abs(), self.Y.abs(), self.Z.abs());
    }

    pub fn Dot(&self, v: &Vector) -> Float {
        return self.X * v.X + self.Y * v.Y + self.Z * v.Z;
    }

    pub fn Cross(&self, v: &Vector) -> Vector {
        let x = self.Y * v.Z - self.Z * v.Y;
        let y = self.Z * v.X - self.X * v.Z;
        let z = self.X * v.Y - self.Y * v.X;
        return V(x, y, z);
    }

    pub fn Add(&self, v: &Vector) -> Vector {
        return V(self.X + v.X, self.Y + v.Y, self.Z + v.Z);
    }

    pub fn Sub(&self, v: &Vector) -> Vector {
        return V(self.X - v.X, self.Y - v.Y, self.Z - v.Z);
    }

    pub fn Mul(&self, v: &Vector) -> Vector {
        return V(self.X * v.X, self.Y * v.Y, self.Z * v.Z);
    }

    pub fn Div(&self, v: &Vector) -> Vector {
        return V(self.X / v.X, self.Y / v.Y, self.Z / v.Z);
    }

    pub fn Rem(&self, v: &Vector) -> Vector {
        let x = self.X - v.X * (self.X / v.X).floor();
        let y = self.Y - v.Y * (self.Y / v.Y).floor();
        let z = self.Z - v.Z * (self.Z / v.Z).floor();
        return V(x, y, z);
    }

    pub fn AddScalar(&self, s: Float) -> Vector {
        return V(self.X + s, self.Y + s, self.Z + s);
    }

    pub fn SubScalar(&self, s: Float) -> Vector {
        return V(self.X - s, self.Y - s, self.Z - s);
    }

    pub fn MulScalar(&self, s: Float) -> Vector {
        return V(self.X * s, self.Y * s, self.Z * s);
    }

    pub fn DivScalar(&self, s: Float) -> Vector {
        return V(self.X / s, self.Y / s, self.Z / s);
    }

    pub fn Min(&self, v: &Vector) -> Vector {
        return V(self.X.min(v.X), self.Y.min(v.Y), self.Z.min(v.Z));
    }

    pub fn Max(&self, v: &Vector) -> Vector {
        return V(self.X.max(v.X), self.Y.max(v.Y), self.Z.max(v.Z));
    }

    pub fn MinAxis(&self) -> Vector {
        let (x, y, z) = (self.X.abs(), self.Y.abs(), self.Z.abs());
        match (x <= y, y <= z) {
             (true,  true) => return V(1.0, 0.0, 0.0),
             (false, true) => return V(0.0, 1.0, 0.0),
             _             => return V(0.0, 0.0, 1.0),
        }
    }

    pub fn MinComponent(&self) -> Float {
        return self.X.min(self.Y).min(self.Z);
    }

    pub fn MaxComponent(&self) -> Float {
        return self.X.max(self.Y).max(self.Z);
    }

    pub fn Reflect(&self, v: &Vector) -> Vector {
        return v.Sub(&self.MulScalar(2.0 * self.Dot(v)));
    }

    pub fn Refract(&self, v: &Vector, n1: Float, n2: Float) -> Vector {
        let nr = n1 / n2;
        let cosI = -self.Dot(v);
        let sinT2 = nr * nr * (1.0 - cosI * cosI);
        if sinT2 > 1.0 {
            return V(0.0, 0.0, 0.0);
        }
        let cosT = (1.0 - sinT2).sqrt();
        return v.MulScalar(nr).Add(&self.MulScalar(nr * cosI - cosT));
    }

    pub fn Reflectance(&self, v: &Vector, n1: Float, n2: Float) -> Float {
        let nr = n1 / n2;
        let cosI = -self.Dot(v);
        let sinT2 = nr * nr * (1.0 - cosI * cosI);
        if sinT2 > 1.0 {
            return 1.0;
        }
        let cosT = (1.0 - sinT2).sqrt();
        let rOrth = (n1 * cosI - n2 * cosT) / (n1 * cosI + n2 * cosT);
        let rPar = (n2 * cosI - n1 * cosT) / (n2 * cosI + n1 * cosT);
        return (rOrth * rOrth + rPar * rPar) / 2.0;
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        self.Neg()
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        self.Add(&v)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector {
        self.Sub(&v)
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector {
        self.Mul(&v)
    }
}

impl ops::Div<Vector> for Vector {
    type Output = Vector;
    fn div(self, v: Vector) -> Vector {
        self.Div(&v)
    }
}

impl ops::Rem<Vector> for Vector {
    type Output = Vector;
    fn rem(self, v: Vector) -> Vector {
        self.Rem(&v)
    }
}

impl ops::Add<Float> for Vector {
    type Output = Vector;
    fn add(self, s: Float) -> Vector {
        self.AddScalar(s)
    }
}

impl ops::Sub<Float> for Vector {
    type Output = Vector;
    fn sub(self, s: Float) -> Vector {
        self.SubScalar(s)
    }
}

impl ops::Mul<Float> for Vector {
    type Output = Vector;
    fn mul(self, s: Float) -> Vector {
        self.MulScalar(s)
    }
}

impl ops::Div<Float> for Vector {
    type Output = Vector;
    fn div(self, s: Float) -> Vector {
        self.DivScalar(s)
    }
}

impl ops::Index<Axis> for Vector {
    type Output = Float;
    fn index(&self, axis: Axis) -> &Float {
        match axis {
            Axis::X => &self.X,
            Axis::Y => &self.Y,
            Axis::Z => &self.Z,
        }
    }
}
