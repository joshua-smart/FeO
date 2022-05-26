use crate::maths::Matrix4x4;

pub trait Transformable {
    fn transform(&self, frame: &Matrix4x4, translate: bool) -> Self;
}
