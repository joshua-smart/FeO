use crate::maths::Vector3;
use crate::data_structures::IntersectionPayload;
use crate::data_structures::Color;
use crate::data_structures::ScatterPayload;

pub trait Material {
    fn emmission(&self, payload: &IntersectionPayload) -> Color;

    fn scatter(&self, payload: &IntersectionPayload, incoming_direction: Vector3) -> Option<ScatterPayload>;

    fn transmission(&self, payload: &IntersectionPayload, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color;

    fn scattering_pdf(&self, payload: &IntersectionPayload, incoming_direction: Vector3, outgoing_direction: Vector3) -> f64;
}
