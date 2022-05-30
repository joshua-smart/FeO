use crate::maths::Vector3;
use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;

pub struct Plane {
    position: Vector3,
    normal: Vector3,
    a_basis: Vector3,
    b_basis: Vector3,

    material_id: usize,
}

impl RenderObject for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        if self.normal * ray.direction == 0.0 { return None; }

        let distance = ((self.position - ray.origin) * self.normal) / (self.normal * ray.direction);
        if distance <= 0.0 { return None; }

        let position = ray.at(distance);
        let a_dist = (position - self.position) * self.a_basis;
        let u = if a_dist >= 0.0 { a_dist % 1.0 } else { a_dist % 1.0 + 1.0 };
        let b_dist = (position - self.position) * self.b_basis;
        let v = if b_dist >= 0.0 { b_dist % 1.0 } else { b_dist % 1.0 + 1.0 };
        Some(IntersectionPayload { position, distance, normal: self.normal.clone(), material_id: self.material_id, u, v })
    }

    fn bounds(&self) -> Bounds {
        Bounds::Full
    }
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, material_id: usize) -> Box<Plane> {
        let unit_normal = normal.normalise();
        let b_basis = Vector3::cross(&unit_normal, &Vector3 (1.0, 0.0, 0.0)).normalise();
        let a_basis = Vector3::cross(&b_basis, &unit_normal);
        Box::new(Plane { position, normal: unit_normal, a_basis, b_basis, material_id })
    }
}
