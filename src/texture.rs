use interaction::Interaction;
use spectrum::Spectrum;

pub trait Texture : Sync + Send {
    type Output;

    fn evaluate(&self, i: &Interaction) -> Self::Output;
}

pub type TextureFloat = Texture<Output = f32>;
pub type TextureSpectrum = Texture<Output = Spectrum>;

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
