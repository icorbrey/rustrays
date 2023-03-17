use math::{color::Color, vector3::Vector3};
use physical::{
    canvas::Canvas,
    light::Light,
    object::Object,
    scene::Scene,
    shader::{Shader, VisualProperties},
};
use render::trace::render;

mod math;
mod physical;
mod render;

fn main() {
    let scene = Scene::new()
        .add_canvas(Canvas::new(1000, 1000))
        .add_viewport(Vector3::new(1, 1, 1), Vector3::new(0, 0, 0))
        .add_light(Light::Ambient(0.2))
        .add_light(Light::Point(0.6, Vector3::new(2, 1, 0)))
        .add_light(Light::Directional(0.2, Vector3::new(1, 4, 4)))
        .add_object(Object::Sphere(
            Vector3::new(0, -1, 3),
            1.0,
            Shader::Lit(VisualProperties {
                color: Color::new(255, 0, 0),
                specular_reflection: Some(500.0),
            }),
        ))
        .add_object(Object::Sphere(
            Vector3::new(2, 0, 4),
            1.0,
            Shader::Lit(VisualProperties {
                color: Color::new(0, 0, 255),
                specular_reflection: Some(500.0),
            }),
        ))
        .add_object(Object::Sphere(
            Vector3::new(-2, 0, 4),
            1.0,
            Shader::Lit(VisualProperties {
                color: Color::new(0, 255, 0),
                specular_reflection: Some(10.0),
            }),
        ))
        .add_object(Object::Sphere(
            Vector3::new(0, -5001, 0),
            5000.0,
            Shader::Lit(VisualProperties {
                color: Color::new(255, 255, 0),
                specular_reflection: Some(1000.0),
            }),
        ))
        .crystalize();

    match render(scene) {
        Ok(()) => println!("Saved successfully."),
        Err(e) => println!("Could not save: {}", e),
    }
}
