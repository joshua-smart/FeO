use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::maths::Vector3;
use crate::shapes::Bounds;

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64,
    material_id: usize
}

impl RenderObject for XZRect {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        let t = (self.y - ray.origin.1) / ray.direction.1;
        if t < 0.0 { return None; }
        let x = ray.origin.0 + t * ray.direction.0;
        let z = ray.origin.2 + t * ray.direction.2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 { return None; }
        let position = ray.at(t);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let normal = Vector3 (0.0, 1.0, 0.0);
        Some(IntersectionPayload { distance: t, position, normal, material_id: self.material_id, u, v})
    }

    fn bounds(&self) -> Bounds {
        Bounds::BoundingBox(Vector3 (self.x0, self.y - 1e-5, self.z0), Vector3 (self.x1, self.y + 1e-5, self.z1))
    }
}

impl XZRect {
    pub fn new(x0: f64, z0: f64, x1: f64, z1: f64, y: f64, material_id: usize) -> Box<XZRect> {
        Box::new(XZRect { x0, x1, z0, z1, y, material_id })
    }
}
