use core::f64::consts::PI;
use fe_o::cameras::PerspectiveCamera;
use fe_o::data_structures::Color;
use fe_o::data_structures::Scene;
use fe_o::materials as M;
use fe_o::maths::Matrix4x4;
use fe_o::maths::Vector3;
use fe_o::shapes as S;
use fe_o::textures as T;
use fe_o::traits::Material;
use fe_o::traits::RenderObject;
use fe_o::Renderer;

fn main() {
    
    let t_red = T::Constant::new(Color (0.65, 0.05, 0.05, 1.0));
    let t_white = T::Constant::new(Color (0.73, 0.73, 0.73, 1.0));
    let t_green = T::Constant::new(Color (0.12, 0.45, 0.15, 1.0));
    let t_light = T::Constant::new(Color (1.0, 1.0, 1.0, 1.0));

    let m_red = M::Lambertian::new(t_red, 0.0);
    let m_white = M::Lambertian::new(t_white, 0.0);
    let m_green = M::Lambertian::new(t_green, 0.0);
    let m_light = M::Lambertian::new(t_light, 15.0);
    let materials: Vec<Box<dyn Material>> = vec![m_red, m_white, m_green, m_light];

    let render_objects: Vec<Box<dyn RenderObject>> = vec![
        S::Plane::new(Vector3 (0.0, 0.0, 0.0), Vector3 (0.0, 0.0, 1.0), 1),
        S::Sphere::new(Vector3 (0.0, 8.0, 1.0), 1.0, 0),
        S::YZRect::new(7.0, 0.0, 9.0, 2.0, -3.0, 3)
    ];


    let scene = Scene::new(render_objects, materials, Color(0.0, 0.0, 0.0, 1.0), 3);

    let transform = Matrix4x4::create_frame_transform(
        Vector3(0.0, 0.0, 1.0),
        Vector3(1.0, 0.0, 0.0),
        Vector3(0.0, 1.0, 0.0),
    );
    let camera = PerspectiveCamera::new(transform, 70.0 * PI / 180.0, 0.0, 8.0);

    let renderer = Renderer::new(scene, camera);
    let image = renderer.render(1920, 1080);
    image.save("output.bmp");
}
