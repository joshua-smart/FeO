use crate::data_structures::Color;
use crate::maths::Vector3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vector3) -> Color;
}
