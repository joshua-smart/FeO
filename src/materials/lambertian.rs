use crate::data_structures::Color;
use crate::traits::Material;
use crate::maths::Vector3;
use crate::traits::Texture;
use crate::data_structures::IntersectionPayload;

pub struct Lambertian {
    albedo: Box<dyn Texture>,
    emmissivity: f64
}

impl Material for Lambertian {
    fn transmission(&self, payload: &IntersectionPayload, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color {
        self.albedo.value(payload.u, payload.v, payload.position) * self.brdf(payload.normal, incoming_direction, outgoing_direction)
    }

    fn brdf(&self, _normal: Vector3, _incoming_direction: Vector3, _outgoing_direction: Vector3) -> f64 {
        1.0
    }

    fn emmission(&self, payload: &IntersectionPayload) -> Color {
        self.albedo.value(payload.u, payload.v, payload.position) * self.emmissivity
    }

    fn generate_direction(&self, normal: Vector3, _incoming_direction: Vector3) -> Vector3 {
        normal + Vector3::random_unit()
    }
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>, emmissivity: f64) -> Lambertian {
        Lambertian { albedo, emmissivity }
    }
}
