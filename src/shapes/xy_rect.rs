use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::maths::Vector3;
use crate::shapes::Bounds;

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    material_id: usize
}

impl RenderObject for XYRect {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        let t = (self.z - ray.origin.2) / ray.direction.2;
        if t < 0.0 { return None; }
        let x = ray.origin.0 + t * ray.direction.0;
        let y = ray.origin.1 + t * ray.direction.1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { return None; }
        let position = ray.at(t);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let normal = Vector3 (0.0, 0.0, 1.0);
        Some(IntersectionPayload { distance: t, position, normal, material_id: self.material_id, u, v})
    }

    fn bounds(&self) -> Bounds {
        Bounds::BoundingBox(Vector3 (self.x0, self.y0, self.z - 1e-5), Vector3 (self.x1, self.y1, self.z + 1e-5))
    }
}

impl XYRect {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64, z: f64, material_id: usize) -> Box<XYRect> {
        Box::new(XYRect { x0, x1, y0, y1, z, material_id })
    }
}
