use crate::traits::RenderObject;
use crate::shapes::Bounds;
use std::cmp::Ordering;
use rand::Rng;
use crate::maths::Vector3;
use crate::data_structures::IntersectionPayload;
use crate::data_structures::Ray;

pub enum BVH {
    Node(Box<BVH>, Box<BVH>, Bounds),
    Shape(Box<dyn RenderObject>, Bounds)
}

impl BVH {
    pub fn intersect(&self, ray: &Ray) -> Option<IntersectionPayload> {
        match self {
            // If intersection the Shape type, check bounds then intersect enclosed render_object
            BVH::Shape(render_object, bounds) => {
                if !bounds.intersect(ray) { return None; }
                render_object.intersect(ray)
            },
            BVH::Node(left, right, bounds) => {
                if !bounds.intersect(ray) { return None; }
                match (left.intersect(ray), right.intersect(ray)) {
                    (None, None) => None,
                    (Some(payload), None) | (None, Some(payload)) => {
                        if payload.distance > 1e-3 { Some(payload) } else { None }
                    },
                    (Some(payload_a), Some(payload_b)) => {
                        if payload_a.distance < payload_b.distance { return Some(payload_a); }
                        Some(payload_b)
                    }
                }
            }
        }
    }

    pub fn new(mut render_objects: Vec<Box<dyn RenderObject>>) -> BVH {
        if render_objects.len() == 1 {
            let render_object = render_objects.remove(0);
            let bounds = render_object.bounds();
            BVH::Shape(render_object, bounds)
        } else {
            let axis = rand::thread_rng().gen_range(0..3);
    
            render_objects.sort_by(|a, b| {
                let box_a = a.bounds();
                let box_b = b.bounds();

                match (box_a, box_b) {
                    (Bounds::Full, Bounds::Full) => Ordering::Equal,
                    (Bounds::Full, _) => Ordering::Greater,
                    (_, Bounds::Full) => Ordering::Less,
                    (Bounds::BoundingBox(min_a, _), Bounds::BoundingBox(min_b, _)) => min_a[axis].partial_cmp(&min_b[axis]).unwrap() 
                }
            });

            let mid = render_objects.len() / 2;
            let right_objects = render_objects.split_off(mid);
            let left = Box::new(BVH::new(render_objects));
            let right = Box::new(BVH::new(right_objects));
            let bounds = BVH::surrounding_box(left.bounds(), right.bounds());

            BVH::Node(left, right, bounds)
        }
    }

    fn bounds(&self) -> &Bounds {
        match self {
            BVH::Node(_, _, bounds) => &bounds,
            BVH::Shape(_, bounds) => &bounds,
        }
    }

    fn surrounding_box(box_a: &Bounds, box_b: &Bounds) -> Bounds {
        match (box_a, box_b) {
            (Bounds::Full, _) => Bounds::Full,
            (_, Bounds::Full) => Bounds::Full,
            (Bounds::BoundingBox(min_a, max_a), Bounds::BoundingBox(min_b, max_b)) => {
                let min = Vector3::min(&min_a, &min_b);
                let max = Vector3::max(&max_a, &max_b);

                Bounds::BoundingBox(min, max)
            }
        }
    }
}

