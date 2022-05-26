use crate::data_structures::Ray;

pub trait Camera {
    fn generate_ray(&self, width: usize, height: usize, x: f64, y: f64) -> Ray;
}
