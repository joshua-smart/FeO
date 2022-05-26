use crate::data_structures::Color;
use crate::traits::Material;
use crate::maths::Vector3;

pub struct DiffuseMaterial {
    albedo: Color,
    emmissivity: f64
}

impl Material for DiffuseMaterial {
    fn transmission(&self, normal: Vector3, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color {
        self.albedo * self.brdf(normal, incoming_direction, outgoing_direction)
    }

    fn brdf(&self, _normal: Vector3, _incoming_direction: Vector3, _outgoing_direction: Vector3) -> f64 {
        1.0
    }

    fn emmission(&self) -> Color {
        self.albedo * self.emmissivity
    }

    fn generate_direction(&self, normal: Vector3, _incoming_direction: Vector3) -> Vector3 {
        normal + Vector3::random_unit()
    }
}

impl DiffuseMaterial {
    pub fn new(albedo: Color, emmissivity: f64) -> DiffuseMaterial {
        DiffuseMaterial { albedo, emmissivity }
    }
}
