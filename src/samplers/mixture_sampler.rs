use crate::traits::Sampler;
use crate::maths::Vector3;
use rand::random;

pub struct MixtureSampler<'a> {
    pdf_a: &'a dyn Sampler,
    pdf_b: &'a dyn Sampler,
}

impl<'a> Sampler for MixtureSampler<'a> {
    fn value(&self, direction: Vector3) -> f64 {
        0.5 * self.pdf_a.value(direction) + 0.5 * self.pdf_b.value(direction)
    }

    fn generate(&self) -> Vector3 {
        if random::<f64>() < 0.5 {
            self.pdf_a.generate()
        } else {
            self.pdf_b.generate()
        }
    }
}

impl<'a> MixtureSampler<'a> {
    pub fn new(pdf_a: &'a dyn Sampler, pdf_b: &'a dyn Sampler) -> MixtureSampler<'a> {
        MixtureSampler { pdf_a, pdf_b }
    }
}
