use std::ops::*;
use crate::maths::Vector3;
use crate::traits::Transformable;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    values: [f64; 16]
}

impl Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Self::Output {
        let mut new_values = [0.0; 16];
        for i in 0..16 { new_values[i] = self.values[i] + rhs.values[i]; }
        Matrix4x4 { values: new_values }
    }
}

impl Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Self::Output {
        self + (rhs * (-1.0))
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut new_values = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                let mut val = 0.0;
                for k in 0..4 {
                    val += self[(i, k)] * rhs[(k, j)];
                }
                new_values[i * 4 + j] = val;
            }
        }
        Matrix4x4 { values: new_values }
    }
}

impl Mul<f64> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, scalar: f64) -> Self::Output {
        let new_values = self.values.map(|a| a * scalar);
        Matrix4x4 { values: new_values }
    }
}

impl Mul<Matrix4x4> for f64 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        rhs * self
    }
}

impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0 * 4 + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.0 * 4 + index.1]
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        let mut equal = true;
        for i in 0..16 {
            equal &= self.values[i] == other.values[i];
        }
        equal
    }
}

impl Matrix4x4 {
    pub fn new(values: [f64; 16]) -> Matrix4x4 {
        Matrix4x4 { values }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new([1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn from_basis(i: Vector3, j: Vector3, k: Vector3) -> Matrix4x4 {
        let basis = [i, j, k];
        let mut matrix = Matrix4x4::identity();
        for i in 0..3 {
            let v = basis[i];
            for j in 0..3 {
                matrix[(j, i)] = v[j];
            }
        }
        matrix
    }

    // create a matrix representing an normalised orthoganal basis with a specific origin, 
    // preserves direction of the supplied i_basis parameter
    pub fn create_frame_transform(origin: Vector3, i_basis: Vector3, j_basis: Vector3) -> Matrix4x4 {
        let ih = i_basis.normalise();
        let kh = Vector3::cross(&ih, &j_basis).normalise();
        let jh = Vector3::cross(&kh, &ih);
        let basis = [ih, jh, kh, origin];
        let mut matrix = Matrix4x4::identity();
        for i in 0..4 {
            let v = basis[i];
            for j in 0..3 {
                matrix[(j, i)] = v[j];
            }
        }
        matrix
    }

    pub fn transform<T: Transformable>(&self, t: &T, translate: bool) -> T {
        t.transform(&self, translate)
    }

    pub fn from_i_basis(i_basis: Vector3) -> Matrix4x4 {
        let j_temp = if i_basis != Vector3 (0.0, 1.0, 0.0) { Vector3 (0.0, 1.0, 0.0) } else { Vector3 (-1.0, 0.0, 0.0) };
        let k_basis = Vector3::cross(&i_basis, &j_temp).normalise();
        let j_basis = Vector3::cross(&k_basis, &i_basis);

        Matrix4x4::from_basis(i_basis, j_basis, k_basis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        let result = Matrix4x4::identity()[(0, 0)];
        assert_eq!(result, 1.0)
    }

    #[test]
    fn from_i_basis() {
        let i_basis = Vector3 (1.0, 0.0, 0.0);
        let result = Matrix4x4::from_i_basis(i_basis);
        assert_eq!(result, Matrix4x4::identity());

        let i2_basis = Vector3 (0.0, 1.0, 0.0);
        let result = Matrix4x4::from_i_basis(i2_basis);
        
        assert_eq!(result.transform(&Vector3 (1.0, 0.0, 0.0), false), Vector3 (0.0, 1.0, 0.0));
        assert_eq!(result.transform(&Vector3 (0.0, 1.0, 0.0), false), Vector3 (-1.0, 0.0, 0.0));
        assert_eq!(result.transform(&Vector3 (0.0, 0.0, 1.0), false), Vector3 (0.0, 0.0, 1.0));

        let i3_basis = Vector3 (1.0, 1.0, 0.0).normalise();
        let result = Matrix4x4::from_i_basis(i3_basis);

        assert_eq!(result.transform(&Vector3 (1.0, 0.0, 0.0), false), i3_basis);
    }
}
