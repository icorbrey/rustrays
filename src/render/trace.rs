use image::ImageResult;

use crate::{
    math::{color::Color, raycast::Raycast, vector3::Vector3},
    physical::{scene::Scene, shader::compute_shading},
};

const REFLECTION_DEPTH: u8 = 3;

pub fn render(mut scene: Scene) -> ImageResult<()> {
    for x in (-1 * scene.canvas.width as i32 / 2)..(scene.canvas.width as i32 / 2) {
        for y in (-1 * scene.canvas.height as i32 / 2)..(scene.canvas.height as i32 / 2) {
            let (origin, direction) = get_initial_ray(&scene, x, y);
            let color = trace_ray(
                &scene,
                origin,
                direction,
                (scene.viewport.z, f64::INFINITY),
                REFLECTION_DEPTH,
            );
            scene.canvas.write_pixel(x, y, color);
        }
    }
    scene.canvas.save("test.png")
}

fn trace_ray(
    scene: &Scene,
    origin: Vector3,
    direction: Vector3,
    range: (f64, f64),
    depth: u8,
) -> Color {
    if let Some(raycast) = Raycast::compute(&scene, origin, direction, range) {
        let local_color = compute_shading(scene, raycast);

        if depth <= 0 {
            return local_color;
        }

        if let Some(reflectivity) = raycast.object.get_shader().reflectivity {
            let reflected_color = trace_ray(
                scene,
                raycast.point,
                raycast.reflection,
                (0.001, f64::INFINITY),
                depth - 1,
            );
            local_color * (1.0 - reflectivity) + reflected_color * reflectivity
        } else {
            local_color
        }
    } else {
        scene.background_color
    }
}

fn get_initial_ray(scene: &Scene, x: i32, y: i32) -> (Vector3, Vector3) {
    let origin = scene.origin;
    let direction = Vector3::new(
        x as f64 * scene.viewport.x / scene.canvas.width as f64,
        y as f64 * scene.viewport.y / scene.canvas.height as f64,
        scene.viewport.z,
    );
    (origin, direction)
}
