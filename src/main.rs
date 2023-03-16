use canvas::Canvas;
use light::Light;
use math::{color::Color, vector3::Vector3};
use object::Object;
use scene::build_scene;
use shader::Shader;
use trace::render;

mod canvas;
mod light;
mod math;
mod object;
mod scene;
mod shader;
mod trace;

fn main() {
    let scene = build_scene()
        .add_canvas(Canvas::new((1000, 1000)))
        .add_viewport(Vector3::new(1, 1, 1), Vector3::new(0, 0, 0))
        .add_light(Light::Ambient(0.2))
        .add_light(Light::Point(0.6, Vector3::new(2, 1, 0)))
        .add_light(Light::Directional(0.6, Vector3::new(1, 4, 4)))
        .add_object(Object::Sphere(
            Vector3::new(0, -1, 3),
            1.0,
            Shader::Lit(Color::new(255, 0, 0)),
        ))
        .add_object(Object::Sphere(
            Vector3::new(2, 0, 4),
            1.0,
            Shader::Lit(Color::new(0, 0, 255)),
        ))
        .add_object(Object::Sphere(
            Vector3::new(-2, 0, 4),
            1.0,
            Shader::Lit(Color::new(0, 255, 0)),
        ))
        .add_object(Object::Sphere(
            Vector3::new(0, -5001, 0),
            5000.0,
            Shader::Lit(Color::new(255, 255, 0)),
        ))
        .crystalize();

    if let Err(e) = render(scene) {
        println!("Could not save: {}", e)
    }
}
