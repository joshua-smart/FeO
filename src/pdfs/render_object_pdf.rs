use crate::traits::PDF;
use crate::traits::RenderObject;
use crate::maths::Vector3;
use crate::data_structures::Ray;

pub struct RenderObjectPDF<'a> {
    origin: Vector3,
    render_object: &'a Box<dyn RenderObject>
}

impl PDF for RenderObjectPDF<'_> {
    fn value(&self, direction: Vector3) -> f64 {
        self.render_object.pdf_value(Ray::new(self.origin, direction))
    }

    fn generate(&self) -> Vector3 {
        self.render_object.random(self.origin)
    }
}

impl RenderObjectPDF<'_> {
    pub fn new(origin: Vector3, render_object: &Box<dyn RenderObject>) -> RenderObjectPDF {
        RenderObjectPDF { origin, render_object }
    }
}
