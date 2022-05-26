use fe_o::maths::Vector3;
use fe_o::cameras::PerspectiveCamera;
use fe_o::maths::Matrix4x4;
use core::f64::consts::PI;
use fe_o::shapes::*;
use fe_o::materials as M;
use fe_o::data_structures::Scene;
use fe_o::data_structures::Color;
use fe_o::Renderer;
use fe_o::traits::Material;
use fe_o::traits::RenderObject;

fn main() {

    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(M::Lambertian::new(Color (0.5, 0.5, 0.5, 1.0), 0.0)),
        Box::new(M::Lambertian::new(Color (0.2, 0.2, 0.9, 1.0), 0.0)),
        Box::new(M::Lambertian::new(Color (0.9, 0.2, 0.2, 1.0), 0.1))
    ];

    let render_objects: Vec<Box<dyn RenderObject>> = vec![
        Box::new(Sphere::new(Vector3 (0.0, 0.0, -1e4), 1e4, 0)),
        Box::new(Sphere::new(Vector3 (0.0, 8.0, 1.0), 1.0, 1)),
        Box::new(Sphere::new(Vector3 (-3.0, 10.0, 1.0), 1.0, 2))
    ];

    let scene = Scene::new(render_objects, materials, Color(0.9, 0.9, 0.7, 1.0), 3);

    let transform = Matrix4x4::create_frame_transform(Vector3 (0.0, 0.0, 2.0), Vector3 (1.0, 0.0, 0.0), Vector3 (0.0, 1.0, -0.2));
    let camera = PerspectiveCamera::new(transform, 70.0 * PI / 180.0);

    let renderer = Renderer::new(scene, camera);
    let image = renderer.render(1024, 512);
    image.save("output.ppm");
}
