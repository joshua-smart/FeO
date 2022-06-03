use crate::traits::PDF;
use crate::maths::Vector3;
use crate::maths::Matrix4x4;
use std::f64::consts::PI;

pub struct CosinePDF {
    normal: Vector3,
    normal_basis: Matrix4x4,
}

impl PDF for CosinePDF {

    fn value(&self, direction: Vector3) -> f64 {
        let cosine = direction * self.normal;
        if cosine <= 0.0 { 1e-5 } else { cosine / PI }
    }

    fn generate(&self) -> Vector3 {
        let direction = Vector3::random_cosine_direction();
        self.normal_basis.transform(&direction, false)
    }
}

impl CosinePDF {
    pub fn new(normal: Vector3) -> CosinePDF {
        let normal_basis = Matrix4x4::from_i_basis(normal);
        CosinePDF { normal, normal_basis }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_zero_pdf() {
        let c = CosinePDF::new(Vector3 (0.0, 1.0, 0.0));
        for _ in 0..10 {
            let d = c.generate();
            let pdf = c.value(d);

            assert_ne!(pdf, 0.0);
        }
    }
}
