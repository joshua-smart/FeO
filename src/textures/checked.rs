use crate::traits::Texture;
use crate::data_structures::Color;
use crate::maths::Vector3;

pub struct Checked {
    color_a: Color,
    color_b: Color,
    scale: f64
}

impl Texture for Checked {
    fn value(&self, _u: f64, _v: f64, p: Vector3) -> Color {
        let sines = (self.scale * p.0).sin() * (self.scale * p.1).sin() * (self.scale * p.2).sin();
        if sines < 0.0 { self.color_a } else { self.color_b }
    }
}

impl Checked {
    pub fn new(color_a: Color, color_b: Color, scale: f64) -> Checked {
        Checked { color_a, color_b, scale }
    }
}
