use crate::maths::Vector3;

pub trait PDF {
    fn value(&self, direction: Vector3) -> f64;
    fn generate(&self) -> Vector3;
}
