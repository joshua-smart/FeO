use crate::data_structures::Scene;
use crate::data_structures::Image;
use crate::traits::Camera;
use rand::random;
use crate::data_structures::Color;
use crossbeam::scope;

pub struct Renderer<C: Camera>{
    scene: Scene,
    camera: C
}

impl<C: Camera> Renderer<C> {
    pub fn new(scene: Scene, camera: C) -> Renderer<C> {
        Renderer { scene, camera }
    }

    pub fn render(self, image_width: usize, image_height: usize) -> Image {
        let thread_samples = 1024 / 4;

        let mut images = Vec::new();

        scope(|s| {
            let mut handles = Vec::new();
            for _i in 0..4 {
                let handle = s.spawn(|_| {
                    let mut image = Image::new(image_width, image_height);

                    for y in 0..image.height {
                        for x in 0..image.width {
                            let mut color = Color (0.0, 0.0, 0.0, 1.0);
                            for _ in 0..thread_samples {
                                let ray = self.camera.generate_ray(image_width, image_height, x as f64 + random::<f64>(), y as f64 + random::<f64>());
                                let c = self.scene.get_color(ray);
                                color = color + c;
                            }
                            image.set_pixel(x, y, &(color / 1024 as f64));
                        }
                        let progress = ((y as f64 / (image.height as f64 - 1.0)) * 100.0) as usize;
                        print!("\r[{}{}] {}%", "#".repeat(progress), "-".repeat(100 - progress), progress);

                    }
                    print!("\n");
                    image
                });
                handles.push(handle);
            }
            for handle in handles {
                let image = handle.join().unwrap();
                images.push(image);
            }
        }).expect("");

        Image::blend(images, image_width, image_height)
    }
}
