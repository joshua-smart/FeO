use crate::data_structures::Color;
use crate::traits::Texture;
use crate::maths::Vector3;

#[derive(Clone, Copy)]
pub struct Constant {
    color: Color,
}

impl Texture for Constant {
    fn value(&self, _u: f64, _v: f64, _p: Vector3) -> Color {
        self.color.clone()
    }
}

impl Constant {
    pub fn new(color: Color) -> Box<Constant> {
        Box::new(Constant { color })
    }
}
