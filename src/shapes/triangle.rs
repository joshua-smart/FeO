use crate::maths::Vector3;
use crate::traits::RenderObject;
use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;

pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,

    normal: Vector3,
    material_id: usize
}

impl RenderObject for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;

        let n = Vector3::cross(&ab, &ac);
        if n * ray.direction == 0.0 { return None; }

        let d = -(n * self.a);
        let t = -(n * ray.origin + d) / (n * ray.direction);

        if t < 0.0 { return None; }
        let p = ray.at(t);
        
        let test_ab = n * Vector3::cross(&(self.b - self.a), &(p - self.a));
        let test_cb = n * Vector3::cross(&(self.c - self.b), &(p - self.b));
        let test_ca = n * Vector3::cross(&(self.a - self.c), &(p - self.c));

        if test_ab < 0.0 || test_cb < 0.0 || test_ca < 0.0 { return None; }
        Some(IntersectionPayload { position: p, distance: t, normal: self.normal, material_id: self.material_id, u: 0.0, v: 0.0 })
    }

    fn bounds(&self) -> Bounds {
        let bound_max = Vector3 ( self.a.0.max(self.b.0).max(self.c.0), self.a.1.max(self.b.1).max(self.c.1), self.a.2.max(self.b.2).max(self.c.2) );
        let bound_min = Vector3 ( self.a.0.min(self.b.0).min(self.c.0), self.a.1.min(self.b.1).min(self.c.1), self.a.2.min(self.b.2).min(self.c.2) );
        Bounds::BoundingBox(bound_min, bound_max)
    }
}

impl Triangle {
    pub fn new(a: Vector3, b: Vector3, c: Vector3, material_id: usize) -> Triangle {
        let normal = Vector3::cross(&(b - a), &(c - a)).normalise();
        Triangle { a, b, c, normal, material_id }
    }
}
