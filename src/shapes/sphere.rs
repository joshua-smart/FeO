use crate::traits::RenderObject;
use crate::maths::Vector3;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;
use core::f64::consts::PI;

#[derive(Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
    material_id: usize,
}

impl RenderObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        let discriminant = (ray.direction * (ray.origin - self.center)).powi(2) - ((ray.origin - self.center).square_magnitude() - self.radius.powi(2));
        if discriminant < 0.0 { return None; }

        let distance = -(ray.direction * (ray.origin - self.center)) - discriminant.sqrt();
        if distance <= 0.0 { return None; }

        let normal = (ray.at(distance) - self.center) / self.radius;
        let position = ray.at(distance);

        let theta = f64::acos(-normal.2);
        let phi = f64::atan2(-normal.1, normal.0);

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        Some(IntersectionPayload { position, distance, normal, material_id: self.material_id, u, v })
    }

    fn bounds(&self) -> Bounds {
        let offset = Vector3 (self.radius, self.radius, self.radius);
        Bounds::BoundingBox (self.center - offset, self.center + offset)
    }
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material_id: usize) -> Sphere {
        Sphere { center, radius, material_id }
    }
}
