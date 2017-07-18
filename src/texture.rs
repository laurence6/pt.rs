use interaction::Interaction;
use spectrum::Spectrum;

pub trait Texture {
    fn evaluate(&self, i: &Interaction) -> Spectrum;
}

pub struct ConstantTexture {
    value: Spectrum,
}

impl ConstantTexture {
    pub fn new(value: Spectrum) -> ConstantTexture {
        ConstantTexture { value }
    }
}

impl Texture for ConstantTexture {
    fn evaluate(&self, i: &Interaction) -> Spectrum {
        self.value
    }
}
