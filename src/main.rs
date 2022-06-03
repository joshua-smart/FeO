use core::f64::consts::PI;
use fe_o::cameras::PerspectiveCamera;
use fe_o::data_structures::Color;
use fe_o::data_structures::Scene;
use fe_o::materials as M;
use fe_o::maths::Matrix4x4;
use fe_o::maths::Vector3;
use fe_o::shapes as S;
use fe_o::textures::*;
use fe_o::traits::Material;
use fe_o::traits::RenderObject;
use fe_o::Renderer;

fn main() {
    
    let t_red = ConstantTexture::new(Color (0.65, 0.05, 0.05, 1.0));
    let t_white = ConstantTexture::new(Color (0.73, 0.73, 0.73, 1.0));
    let t_green = ConstantTexture::new(Color (0.12, 0.45, 0.15, 1.0));
    let t_light = ConstantTexture::new(Color (1.0, 1.0, 1.0, 1.0));

    let m_red = M::Lambertian::new(t_red, 0.0);
    let m_white = M::Lambertian::new(t_white, 0.0);
    let m_green = M::Lambertian::new(t_green, 0.0);
    let m_light = M::Lambertian::new(t_light, 15.0);
    let materials: Vec<Box<dyn Material>> = vec![m_red, m_white, m_green, m_light];

    let render_objects: Vec<Box<dyn RenderObject>> = vec![
        S::XYRect::new(0.0, 0.0, 555.0, 555.0, 0.0, 1),
//        S::XYRect::new(0.0, 0.0, 555.0, 555.0, 555.0, 1),
//        S::XZRect::new(0.0, 0.0, 555.0, 555.0, 555.0, 1),
//        S::YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, 2),
//        S::YZRect::new(0.0, 0.0, 555.0, 555.0, 555.0, 0),
        S::XYRect::new(213.0, 227.0, 343.0, 332.0, 540.0, 3),
        S::Sphere::new(Vector3 (278.0, 278.0, 100.0), 100.0, 0)
    ];


    let scene = Scene::new(render_objects, materials, vec![S::XYRect::new(213.0, 227.0, 343.0, 332.0, 540.0, 3)], Color(0.5, 0.5, 0.5, 1.0), 1);

    let transform = Matrix4x4::create_frame_transform(
        Vector3(278.0, -800.0, 278.0),
        Vector3(1.0, 0.0, 0.0),
        Vector3(0.0, 1.0, 0.0),
    );
    let camera = PerspectiveCamera::new(transform, 40.0 * PI / 180.0, 0.0, 8.0);

    let renderer = Renderer::new(scene, camera);
    let image = renderer.render(600, 600);
    image.save("output.bmp");
}
