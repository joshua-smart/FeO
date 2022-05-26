use crate::maths::Vector3;
use crate::data_structures::Color;

pub trait Material {
    fn transmission(&self, normal: Vector3, incoming_direction: Vector3, outgoing_direction: Vector3) -> Color;

    fn brdf(&self, normal: Vector3, incoming_direction: Vector3, outgoing_direction: Vector3) -> f64;

    fn generate_direction(&self, normal: Vector3, incoming_direction: Vector3) -> Vector3;

    fn emmission(&self) -> Color;
}
