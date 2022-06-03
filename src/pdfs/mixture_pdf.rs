use crate::traits::PDF;
use crate::maths::Vector3;
use rand::random;

pub struct MixturePDF<'a> {
    pdf_a: &'a dyn PDF,
    pdf_b: &'a dyn PDF,
}

impl<'a> PDF for MixturePDF<'a> {
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

impl<'a> MixturePDF<'a> {
    pub fn new(pdf_a: &'a dyn PDF, pdf_b: &'a dyn PDF) -> MixturePDF<'a> {
        MixturePDF { pdf_a, pdf_b }
    }
}
