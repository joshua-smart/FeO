use crate::maths::Vector3;
use crate::traits::Transformable;
use crate::traits::Camera;
use crate::data_structures::Ray;
use crate::maths::Matrix4x4;

pub struct PerspectiveCamera {
    transform: Matrix4x4,
    tan_half_fov: f64,
    aperture: f64,
    focal_depth: f64,
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, image_width: usize, image_height: usize, image_col: f64, image_row: f64) -> Ray {
        let aspect_ratio = (image_height as f64) / (image_width as f64);
        let offset = Vector3::random_in_unit_disk() * self.aperture / 2.0;

        let z = ((image_row / (image_height as f64)) * 2.0 - 1.0) * self.tan_half_fov * self.focal_depth * aspect_ratio;
        let x = ((image_col / (image_width as f64)) * 2.0 - 1.0) * self.tan_half_fov * self.focal_depth;

        let ray = Ray::new(offset, (Vector3 (x, self.focal_depth, -z) - offset).normalise());
        ray.transform(&self.transform, true)
    }
}

impl PerspectiveCamera {
    pub fn new(transform: Matrix4x4, field_of_view: f64, aperture: f64, focal_depth: f64) -> PerspectiveCamera {
        let tan_half_fov = (field_of_view / 2.0).tan();
        PerspectiveCamera { transform, tan_half_fov, aperture, focal_depth }
    }
}
