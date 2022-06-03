use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;
use crate::maths::Vector3;

pub trait RenderObject {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload>;
    fn bounds(&self) -> Bounds;

    fn pdf_value(&self, _ray: Ray) -> f64 {
        panic!("Renderobject::pdf_value not implemented")
    }

    fn random(&self, _origin: Vector3) -> Vector3 {
        panic!("RenderObject::random not implemented")
    }
}
