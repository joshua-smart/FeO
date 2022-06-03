use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::maths::Vector3;
use crate::shapes::Bounds;
use rand::random;

pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64,
    material_id: usize
}

impl RenderObject for YZRect {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        let t = (self.x - ray.origin.0) / ray.direction.0;
        if t < 0.0 { return None; }
        let y = ray.origin.1 + t * ray.direction.1;
        let z = ray.origin.2 + t * ray.direction.2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 { return None; }
        let position = ray.at(t);
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let normal = Vector3 (1.0, 0.0, 0.0);
        Some(IntersectionPayload { distance: t, position, normal, material_id: self.material_id, u, v})
    }

    fn bounds(&self) -> Bounds {
        Bounds::BoundingBox(Vector3 (self.x - 1e-5, self.y0, self.z0), Vector3 (self.x + 1e-5, self.y1, self.z1))
    }

    fn pdf_value(&self, ray: Ray) -> f64 {
        match self.intersect(&ray) {
            None => 0.0,
            Some(payload) => {
                let area = (self.y1 - self.y0) * (self.z1 - self.z0);
                let square_distance = payload.distance.powi(2);
                let cosine = (ray.direction * payload.normal).abs();

                if cosine == 0.0 { return 0.0; }

                square_distance / (cosine * area)
            }
        }
    }

    fn random(&self, origin: Vector3) -> Vector3 {
        let y = random::<f64>() * (self.y1 - self.y0) + self.y0;
        let z = random::<f64>() * (self.z1 - self.z0) + self.z0;

        (Vector3 (self.x, y, z) - origin).normalise()
    }
}

impl YZRect {
    pub fn new(y0: f64, z0: f64, y1: f64, z1: f64, x: f64, material_id: usize) -> Box<YZRect> {
        Box::new(YZRect { y0, y1, z0, z1, x, material_id })
    }
}
