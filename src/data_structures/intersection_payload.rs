use crate::maths::Vector3;

pub struct IntersectionPayload {
    pub position: Vector3,
    pub distance: f64,
    pub normal: Vector3,
    pub material_id: usize,
    pub u: f64,
    pub v: f64
}
