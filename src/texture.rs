use interaction::Interaction;
use spectrum::Spectrum;
use transform::Transform;
use vector::{Point3f, Point2f};

pub trait Texture : Sync + Send {
    type Output;

    fn evaluate(&self, i: &Interaction) -> Self::Output;
}

pub type TextureFloat = Texture<Output = f32>;
pub type TextureSpectrum = Texture<Output = Spectrum>;

pub trait TextureMapping2D : Sync + Send {
    fn map(&self, i: &Interaction) -> Point2f;
}

pub trait TextureMapping3D : Sync + Send {
    fn map(&self, i: &Interaction) -> Point3f;
}

pub struct ConstantTexture<T> where T: Copy + Sync + Send {
    value: T,
}

impl<T> ConstantTexture<T> where T: Copy + Sync + Send {
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl<T> Texture for ConstantTexture<T> where T: Copy + Sync + Send {
    type Output = T;
    fn evaluate(&self, i: &Interaction) -> T {
        self.value
    }
}

pub struct CheckerboardTexture<T> where T: Copy + Sync + Send {
    mapping: Box<TextureMapping2D>,
    tex1: T,
    tex2: T,
}

impl<T> CheckerboardTexture<T> where T: Copy + Sync + Send {
    pub fn new<M>(mapping: M, tex1: T, tex2: T) -> CheckerboardTexture<T> where M: 'static + TextureMapping2D {
        CheckerboardTexture {
            mapping: Box::new(mapping),
            tex1,
            tex2,
        }
    }
}

impl<T> Texture for CheckerboardTexture<T> where T: Copy + Sync + Send {
    type Output = T;
    fn evaluate(&self, i: &Interaction) -> T {
        let st = self.mapping.map(i);
        if (st.x.floor() as i64 + st.y.floor() as i64) % 2 == 0 {
            return self.tex1;
        } else {
            return self.tex2;
        }
    }
}

pub struct Checkerboard3DTexture<T> where T: Copy + Sync + Send {
    mapping: Box<TextureMapping3D>,
    tex1: T,
    tex2: T,
}

impl<T> Checkerboard3DTexture<T> where T: Copy + Sync + Send {
    pub fn new<M>(mapping: M, tex1: T, tex2: T) -> Checkerboard3DTexture<T> where M: 'static + TextureMapping3D {
        Checkerboard3DTexture {
            mapping: Box::new(mapping),
            tex1,
            tex2,
        }
    }
}

impl<T> Texture for Checkerboard3DTexture<T> where T: Copy + Sync + Send {
    type Output = T;
    fn evaluate(&self, i: &Interaction) -> T {
        let p = self.mapping.map(i);
        if (p.x.floor() as i64 + p.y.floor() as i64 + p.z.floor() as i64) % 2 == 0 {
            return self.tex1;
        } else {
            return self.tex2;
        }
    }
}

pub struct UVMapping2D {
    su: f32,
    sv: f32,
    du: f32,
    dv: f32,
}

impl UVMapping2D {
    pub fn new(su: f32, sv: f32, du: f32, dv: f32) -> UVMapping2D {
        UVMapping2D { su, sv, du, dv }
    }
}

impl TextureMapping2D for UVMapping2D {
    fn map(&self, i: &Interaction) -> Point2f {
        Point2f::new(
            i.uv[0] * self.su + self.du,
            i.uv[1] * self.sv + self.dv,
        )
    }
}

pub struct IdentityMapping3D {
    world_to_texture: Transform,
}

impl IdentityMapping3D {
    pub fn new(world_to_texture: Transform) -> IdentityMapping3D {
        IdentityMapping3D { world_to_texture }
    }
}

impl TextureMapping3D for IdentityMapping3D {
    fn map(&self, i: &Interaction) -> Point3f {
        self.world_to_texture.apply(&i.p)
    }
}
