use crate::data_structures::Scene;
use crate::data_structures::Image;
use crate::traits::Camera;
use rand::random;
use crate::data_structures::Color;

pub struct Renderer<C: Camera>{
    scene: Scene,
    camera: C
}

impl<C: Camera> Renderer<C> {
    pub fn new(scene: Scene, camera: C) -> Renderer<C> {
        Renderer { scene, camera }
    }

    pub fn render(self, image_width: usize, image_height: usize) -> Image {
        let samples = 16;

        let mut image = Image::new(image_width, image_height);

        for y in 0..image.height {
            for x in 0..image.width {
                let mut color = Color (0.0, 0.0, 0.0, 1.0);
                for _ in 0..samples {
                    let ray = self.camera.generate_ray(image_width, image_height, x as f64 + random::<f64>(), y as f64 + random::<f64>());
                    let c = self.scene.get_color(ray);
                    color = color + c;
                }
                image.set_pixel(x, y, &(color / samples as f64));
            }
            let progress = ((y as f64 / (image.height as f64 - 1.0)) * 100.0) as usize;
            print!("\r[{}{}] {}%", "#".repeat(progress), "-".repeat(100 - progress), progress)

        }
        print!("\n");
        image
    }
}
