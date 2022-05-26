use crate::maths::Vector3;
use crate::traits::Transformable;
use crate::maths::Matrix4x4;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}

impl Transformable for Ray {
    fn transform(&self, frame: &Matrix4x4, translate: bool) -> Self {
        let origin = self.origin.transform(frame, translate);
        let direction = self.direction.transform(frame, false);
        Ray { origin, direction }
    }
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
}
