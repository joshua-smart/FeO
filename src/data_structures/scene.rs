use crate::traits::RenderObject;
use crate::data_structures::Color;
use crate::data_structures::Ray;
use crate::traits::Material;
use crate::shapes::BVH;

pub struct Scene {
    bounding_volume_hierarchy: BVH,
    materials: Vec<Box<dyn Material>>,
    background_color: Color,

    max_depth: usize
}

impl Scene {

    pub fn get_color(&self, ray: Ray) -> Color {
        self.trace(ray, 0)
    }

    fn trace(&self, ray: Ray, depth: usize) -> Color {
        if depth >= self.max_depth { return self.background_color; }

        let payload_option = self.bounding_volume_hierarchy.intersect(&ray);
        match payload_option {
            None => self.background_color,
            Some(payload) => {
                let material = &self.materials[payload.material_id];
                let outgoing_direction = material.generate_direction(payload.normal, ray.direction);
                let outgoing_ray = Ray { origin: payload.position, direction: outgoing_direction };

                let light_sampled = self.trace(outgoing_ray, depth + 1);
                let light_emmited = material.emmission(&payload);
                let light_transmitted = material.transmission(&payload, ray.direction, outgoing_direction);

                light_sampled * light_transmitted + light_emmited
            }
        }
    }

//    fn get_closest_intersect(&self, ray: Ray) -> Option<IntersectionPayload> {
//        let mut record_payload = None;
//        let mut record_distance = f64::INFINITY;
//
//        for i in 0..self.render_objects.len() {
//            let render_object = &self.render_objects[i];
//            if !render_object.bounding_box().intersect(&ray) { continue }
//            match render_object.intersect(&ray) {
//                None => (),
//                Some(payload) => {
//                    if payload.distance > 1e-3 && payload.distance < record_distance {
//                        record_distance = payload.distance;
//                        record_payload = Some(payload);
//                    }
//                }
//            }
//        }
//
//        record_payload
//    }

    pub fn new(render_objects: Vec<Box<dyn RenderObject>>, materials: Vec<Box<dyn Material>>, background_color: Color, max_depth: usize) -> Scene {
        let bounding_volume_hierarchy = BVH::new(render_objects);
        Scene { bounding_volume_hierarchy, materials, background_color, max_depth }
    }
}
