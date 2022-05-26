use crate::data_structures::Ray;
use crate::data_structures::IntersectionPayload;
use crate::shapes::Bounds;

pub trait RenderObject {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload>;
    fn bounds(&self) -> Bounds;
}
