use crate::data_structures::Color;
use crate::traits::Texture;
use crate::maths::Vector3;

pub struct ConstantTexture {
    color: Color,
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vector3) -> Color {
        self.color
    }
}

impl ConstantTexture {
    pub fn new(color: Color) -> Box<ConstantTexture> {
        Box::new(ConstantTexture { color })
    }
}
