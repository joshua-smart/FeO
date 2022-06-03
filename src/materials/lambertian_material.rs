use crate::traits::Material;
use crate::maths::Vector3;
use crate::traits::Texture;
use crate::data_structures::IntersectionPayload;
use std::f64::consts::PI;
use crate::data_structures::Color;
use crate::data_structures::ScatterPayload;
use crate::samplers::CosineSampler;

pub struct LambertianMaterial {
    pub albedo: Box<dyn Texture>,
    pub emmissivity: f64
}

impl Material for LambertianMaterial {
    fn scattering_pdf(&self, payload: &IntersectionPayload, _incoming_direction: Vector3, outgoing_direction: Vector3) -> f64 {
        let cosine = payload.normal * outgoing_direction;
        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

    fn scatter(&self, payload: &IntersectionPayload, _incoming_direction: Vector3) -> Option<ScatterPayload> {
        let attenuation = self.albedo.value(payload.u, payload.v, payload.position);
        let pdf = CosineSampler::new(payload.normal);
        Some(ScatterPayload { is_specular: false, attenuation, pdf: Box::new(pdf) })
    }

    fn emmission(&self, payload: &IntersectionPayload) -> Color {
        self.albedo.value(payload.u, payload.v, payload.position) * self.emmissivity
    }

    fn transmission(&self, payload: &IntersectionPayload, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color {
        self.albedo.value(payload.u, payload.v, payload.position) * self.scattering_pdf(payload, incoming_direction, outgoing_direction)
    }
}

impl LambertianMaterial {
    pub fn new(albedo: Box<dyn Texture>, emmissivity: f64) -> Box<LambertianMaterial> {
        Box::new(LambertianMaterial { albedo, emmissivity })
    }
}
