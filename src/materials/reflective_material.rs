use crate::traits::Material;
use crate::data_structures::Color;
use crate::maths::Vector3;

pub struct ReflectiveMaterial {
    reflectance: f64
}

impl Material for ReflectiveMaterial {
    fn emmission(&self) -> Color {
        Color (0.0, 0.0, 0.0, 1.0)
    }

    fn transmission(&self, _normal: Vector3, _incoming_direction: Vector3, _outgoing_direction: Vector3) -> Color {
        Color (1.0, 1.0, 1.0, 1.0)
    }

    fn brdf(&self, _normal: Vector3, _incoming_direction: Vector3, _outgoing_direction: Vector3) -> f64 {
        1.0
    }

    fn generate_direction(&self, normal: Vector3, incoming_direction: Vector3) -> Vector3 {
        incoming_direction.reflect(normal) + Vector3::random_unit() * self.reflectance
    }

}

impl ReflectiveMaterial {
    pub fn new(reflectance: f64) -> ReflectiveMaterial {
        ReflectiveMaterial { reflectance }
    }
}
