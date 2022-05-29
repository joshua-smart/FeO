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
        if depth >= self.max_depth { return Color (0.0, 0.0, 0.0, 1.0); }

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

    pub fn new(render_objects: Vec<Box<dyn RenderObject>>, materials: Vec<Box<dyn Material>>, background_color: Color, max_depth: usize) -> Scene {
        let bounding_volume_hierarchy = BVH::new(render_objects);
        Scene { bounding_volume_hierarchy, materials, background_color, max_depth }
    }
}
