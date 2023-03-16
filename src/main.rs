use canvas::Canvas;
use light::Light;
use math::{Color, Transform, Vector3};
use object::Object;
use scene::create_scene;
use shader::Shader;

mod canvas;
mod light;
mod math;
mod object;
mod scene;
mod shader;

fn main() {
    let result = create_scene()
        .add_canvas(Canvas::new((1000, 1000)))
        .add_viewport(Vector3::new(0, 0, 0), Vector3::new(1, 1, 1))
        .add_lights(vec![
            Light::Ambient(0.2),
            Light::Point(0.6, Vector3::new(2, 1, 0)),
            Light::Directional(0.2, Vector3::new(1, 4, 4)),
        ])
        .add_objects(vec![
            Object::Sphere(
                Transform::from_position(Vector3::new(0, -1, 3)),
                1.0,
                Shader::Lit(Color::new(255, 0, 0)),
            ),
            Object::Sphere(
                Transform::from_position(Vector3::new(2, 0, 4)),
                1.0,
                Shader::Lit(Color::new(0, 0, 255)),
            ),
            Object::Sphere(
                Transform::from_position(Vector3::new(-2, 0, 4)),
                1.0,
                Shader::Lit(Color::new(0, 255, 0)),
            ),
            Object::Sphere(
                Transform::from_position(Vector3::new(0, -5001, 0)),
                5000.0,
                Shader::Lit(Color::new(255, 255, 0)),
            ),
        ])
        .render()
        .save("test.png");

    if let Err(e) = result {
        println!("Could not save: {}", e)
    }
}
