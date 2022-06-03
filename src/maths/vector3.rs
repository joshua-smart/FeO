use std::ops::*;
use crate::traits::Transformable;
use crate::maths::Matrix4x4;
use rand::random;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 (pub f64, pub f64, pub f64);

fn component_wise<F: Fn(f64, f64) -> f64>(a: &Vector3, b: &Vector3, f: F) -> Vector3 {
    Vector3 (f(a.0, b.0), f(a.1, b.1), f(a.2, b.2))
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        component_wise(&self, &rhs, |a, b| a + b)
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = f64;
    fn mul(self, rhs: Vector3) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f64) -> Self::Output {
        Vector3 (self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}
impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        v * self
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        self + (rhs * (-1.0))
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        component_wise(&self, &rhs, |a, b| a / b)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Self::Output {
        Vector3 (self / v.0, self / v.1, self / v.2)
    }
}

impl Index<usize> for Vector3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index out of range {} for Vector3", index)
        }
    }
}

impl Transformable for Vector3 {
    fn transform(&self, frame: &Matrix4x4, translate: bool) -> Self {
        let w: f64 = if translate { 1.0 } else { 0.0 };
        return Vector3 (
            self.0 * frame[(0, 0)] + self.1 * frame[(0, 1)] + self.2 * frame[(0, 2)] + w * frame[(0, 3)],
            self.0 * frame[(1, 0)] + self.1 * frame[(1, 1)] + self.2 * frame[(1, 2)] + w * frame[(1, 3)],
            self.0 * frame[(2, 0)] + self.1 * frame[(2, 1)] + self.2 * frame[(2, 2)] + w * frame[(2, 3)]
        );
    }
}

impl Vector3 {
    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 (a.1 * b.2 - a.2 * b.1,
                 a.2 * b.0 - a.0 * b.2,
                 a.0 * b.1 - a.1 * b.0)
    }

    pub fn min(a: &Vector3, b: &Vector3) -> Vector3 {
        component_wise(a, b, |a, b| a.min(b))
    }

    pub fn max(a: &Vector3, b: &Vector3) -> Vector3 {
        component_wise(a, b, |a, b| a.max(b))
    }

    pub fn square_magnitude(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    pub fn normalise(self) -> Vector3 {
        self * (1.0 / self.magnitude())
    }

    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        *self - 2.0 * (normal * *self) * normal
    }

    pub fn random_unit() -> Vector3 {
        let mut v;
        while {
            v = Vector3 (random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0);
            v.square_magnitude() > 1.0
        } {}
        v.normalise()
    }

    pub fn random_in_unit_disk() -> Vector3 {
        let mut v;
        while {
            v = Vector3(random::<f64>() * 2.0 - 1.0, random::<f64>() * 2.0 - 1.0, 0.0);
            v.square_magnitude() > 1.0
        } {}
        v
    }

    pub fn random_cosine_direction() -> Vector3 {
        let r1 = random::<f64>();
        let r2 = random::<f64>();
        let x = (1.0 - r2).sqrt();

        let phi = 2.0 * PI * r1;
        let y = phi.cos() * r2.sqrt();
        let z = phi.sin() * r2.sqrt();

        Vector3 (x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector_a() -> Vector3 {
        Vector3(1.3, 5.5, -0.6)
    }
    fn vector_b() -> Vector3 {
        Vector3(0.0, -4.5, 10.1)
    }

    fn compare_vectors(a: &Vector3, b: &Vector3) -> f64 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
    }

    #[test]
    fn add() {
        let result = vector_a() + vector_b();
        assert_eq!(result, Vector3 (1.3, 1.0, 9.5))
    }
    #[test]
    fn sub() {
        let result = vector_a() - vector_b();
        assert_eq!(result, Vector3 (1.3, 10.0, -10.7))
    }
    #[test]
    fn dot() {
        let result = vector_a() * vector_b();
        assert_eq!(result, -30.81)
    }
    #[test]
    fn cross() {
        let result = Vector3::cross(&vector_a(), &vector_b());
        assert!(compare_vectors(&result, &Vector3 (52.85, -13.13, -5.85)) < 1e-10)
    }
    #[test]
    fn square_magnitude() {
        let result = vector_a().square_magnitude();
        assert!((result - 32.3).abs() < 1e-10)
    }
    #[test]
    fn magnitude() {
        let result = vector_a().magnitude();
        assert!((result - 5.68).abs() < 1e-2)
    }
    #[test]
    fn normalise() {
        let result = vector_a().normalise();
        assert!((1.0 - result.magnitude()).abs() < 1e-10)
    }
    #[test]
    fn random_cosine_direction() {
        for _ in 0..10 {
            let result = Vector3::random_cosine_direction();
            assert!((result.magnitude() - 1.0).abs() < 1e-2);
            assert!(result * Vector3 (1.0, 0.0, 0.0) > 0.0);
        }
    }
}
