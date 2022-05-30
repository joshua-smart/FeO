use crate::maths::Vector3;
use crate::data_structures::Color;
use crate::data_structures::IntersectionPayload;

pub trait Material: Sync + Send {
    fn transmission(&self, payload: &IntersectionPayload, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color;

    fn brdf(&self, normal: Vector3, incoming_direction: Vector3, outgoing_direction: Vector3) -> f64;

    fn generate_direction(&self, normal: Vector3, incoming_direction: Vector3) -> Vector3;

    fn emmission(&self, payload: &IntersectionPayload) -> Color;
}
