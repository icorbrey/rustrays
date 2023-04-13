use math::{color::Color, vector3::Vector3};
use physical::{canvas::Canvas, light::Light, object::Object, scene::Scene, shader::Shader};
use render::trace::render;

mod gui;
mod math;
mod physical;
mod render;

fn main() {
    let scene = Scene::new()
        .add_canvas(Canvas::new(1920, 1080))
        .add_viewport(Vector3::new(16.0 / 9.0, 1, 1), Vector3::new(0, 0, 0))
        .add_background_color(Color::new(40, 40, 40))
        .add_light(Light::Ambient { intensity: 0.2 })
        .add_light(Light::Point {
            intensity: 0.6,
            origin: Vector3::new(2, 1, 0),
        })
        .add_light(Light::Directional {
            intensity: 0.2,
            direction: Vector3::new(1, 4, 4),
        })
        .add_object(Object::Sphere {
            position: Vector3::new(0, -1, 3),
            radius: 1.0,
            shader: Shader {
                color: Color::new(255, 0, 0),
                specular_reflection: Some(500.0),
                reflectivity: Some(0.2),
            },
        })
        .add_object(Object::Sphere {
            position: Vector3::new(2, 0, 4),
            radius: 1.0,
            shader: Shader {
                color: Color::new(0, 0, 255),
                specular_reflection: Some(500.0),
                reflectivity: Some(0.3),
            },
        })
        .add_object(Object::Sphere {
            position: Vector3::new(-2, 0, 4),
            radius: 1.0,
            shader: Shader {
                color: Color::new(0, 255, 0),
                specular_reflection: Some(10.0),
                reflectivity: Some(0.4),
            },
        })
        .add_object(Object::Sphere {
            position: Vector3::new(0, -5001, 0),
            radius: 5000.0,
            shader: Shader {
                color: Color::new(255, 255, 0),
                specular_reflection: Some(1000.0),
                reflectivity: Some(0.5),
            },
        })
        .crystalize();

    gui::initialize();

    match render(scene) {
        Ok(()) => println!("Saved successfully."),
        Err(e) => println!("Could not save: {}", e),
    }
}
