use crate::traits::RenderObject;
use crate::data_structures::Color;
use crate::data_structures::Ray;
use crate::traits::Material;
use crate::shapes::BVH;
use crate::pdfs::RenderObjectPDF;
use crate::pdfs::MixturePDF;
use crate::traits::PDF;
use crate::pdfs::CosinePDF;

pub struct Scene {
    bounding_volume_hierarchy: BVH,
    materials: Vec<Box<dyn Material>>,
    lights: Vec<Box<dyn RenderObject>>,
    background_color: Color,

    max_depth: usize
}

impl Scene {

    pub fn get_color(&self, ray: Ray) -> Color {
        self.trace(ray, 0)
    }

    fn trace(&self, ray: Ray, depth: usize) -> Color {
        if depth > self.max_depth { return Color (0.0, 0.0, 0.0, 1.0); }

        let payload_option = self.bounding_volume_hierarchy.intersect(&ray);
        match payload_option {
            None => self.background_color,
            Some(payload) => {
                let material = &self.materials[payload.material_id];
                let light_emmited = material.emmission(&payload);

                match material.scatter(&payload, ray.direction) {
                    None => light_emmited,
                    Some(scatter) => {
                        let pdf_a = RenderObjectPDF::new(payload.position, &self.lights[0]);
                        let pdf_b = CosinePDF::new(payload.normal);
                        let mix_pdf = MixturePDF::new(&pdf_a, &pdf_b);

                        let outgoing_direction = mix_pdf.generate();
                        let outgoing_ray = Ray { origin: payload.position, direction: outgoing_direction };
                        let pdf_value = mix_pdf.value(outgoing_direction);

                        let light_sampled = self.trace(outgoing_ray, depth + 1);
                        let light_transmitted = material.transmission(&payload, ray.direction, outgoing_direction);

                        if pdf_value != 0.0 { 
                            light_emmited + (light_transmitted * light_sampled) / pdf_value
                        } else {
                            light_emmited
                        }
                    }
                }
            }
        }
    }

    pub fn new(render_objects: Vec<Box<dyn RenderObject>>, materials: Vec<Box<dyn Material>>, lights: Vec<Box<dyn RenderObject>>, background_color: Color, max_depth: usize) -> Scene {
        let bounding_volume_hierarchy = BVH::new(render_objects);
        Scene { bounding_volume_hierarchy, materials, lights, background_color, max_depth }
    }
}
