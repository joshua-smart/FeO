use crate::maths::Vector3;
use crate::data_structures::Ray;

#[derive(Debug)]
pub enum Bounds {
    BoundingBox (Vector3, Vector3),
    Full,
}

impl Bounds {
    pub fn new(a: Vector3, b: Vector3) -> Bounds {
        Bounds::BoundingBox(a, b)
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        match self {
            Bounds::BoundingBox (min_point, max_point) => {
                for a in 0..3 {
                    let inv_d = 1.0 / ray.direction[a];
                    let mut t0 = (min_point[a] - ray.origin[a]) * inv_d;
                    let mut t1 = (max_point[a] - ray.origin[a]) * inv_d;
                    if inv_d < 0.0 {
                        let temp = t0;
                        t0 = t1;
                        t1 = temp;
                    } else { () }
                    if t1 <= t0 { return false; }
                }
                true
            },
            Bounds::Full => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let bounding_box = Bounds::BoundingBox (Vector3 (0.0, 0.0, 0.0), Vector3 (1.0, 1.0, 1.0));
        let ray = Ray { origin: Vector3 (0.5, -1.0, 0.5), direction: Vector3 (0.0, 1.0, 0.0) };

        assert!(bounding_box.intersect(&ray));

        let ray2 = Ray { origin: Vector3 (0.5, -1.0, 0.5), direction: Vector3 (0.0, -1.0, 0.0) };

        assert!(!bounding_box.intersect(&ray2));
    }
}
