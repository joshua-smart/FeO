use crate::traits::Texture;
use crate::data_structures::Color;
use crate::maths::Vector3;

pub struct CheckedTexture {
    color_a: Color,
    color_b: Color,
}

impl Texture for CheckedTexture {
    fn value(&self, u: f64, v: f64, _p: Vector3) -> Color {
        if ((u - 0.5) * (v - 0.5)) < 0.0 { self.color_a } else { self.color_b }
    }
}

impl CheckedTexture {
    pub fn new(color_a: Color, color_b: Color) -> Box<CheckedTexture> {
        Box::new(CheckedTexture { color_a, color_b })
    }
}
