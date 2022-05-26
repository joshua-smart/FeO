use crate::maths::Vector3;
use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;

pub struct Plane {
    position: Vector3,
    normal: Vector3,

    material_id: usize,
}

impl RenderObject for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        if self.normal * ray.direction == 0.0 { return None; }

        let distance = (self.position - ray.origin) * self.normal / (self.normal * ray.direction);
        if distance <= 0.0 { return None; }

        let position = ray.at(distance);
        Some(IntersectionPayload { position, distance, normal: self.normal.clone(), material_id: self.material_id })
    }

    fn bounds(&self) -> Bounds {
        Bounds::Full
    }
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, material_id: usize) -> Plane {
        let unit_normal = normal.normalise();
        Plane { position, normal: unit_normal, material_id }
    }
}
