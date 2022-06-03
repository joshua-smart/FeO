use crate::traits::Sampler;
use crate::traits::RenderObject;
use crate::maths::Vector3;
use crate::data_structures::Ray;

pub struct RenderObjectSampler<'a> {
    origin: Vector3,
    render_object: &'a Box<dyn RenderObject>
}

impl Sampler for RenderObjectSampler<'_> {
    fn value(&self, direction: Vector3) -> f64 {
        self.render_object.pdf_value(Ray::new(self.origin, direction))
    }

    fn generate(&self) -> Vector3 {
        self.render_object.random(self.origin)
    }
}

impl RenderObjectSampler<'_> {
    pub fn new(origin: Vector3, render_object: &Box<dyn RenderObject>) -> RenderObjectSampler {
        RenderObjectSampler { origin, render_object }
    }
}
