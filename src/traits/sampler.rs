use crate::maths::Vector3;

pub trait Sampler {
    fn value(&self, direction: Vector3) -> f64;
    fn generate(&self) -> Vector3;
}
