use image::ImageResult;

use crate::{
    math::{raycast::Raycast, vector3::Vector3},
    physical::{scene::Scene, shader::compute_shading},
};

pub fn render(mut scene: Scene) -> ImageResult<()> {
    for x in (-1 * scene.canvas.width as i32 / 2)..(scene.canvas.width as i32 / 2) {
        for y in (-1 * scene.canvas.height as i32 / 2)..(scene.canvas.height as i32 / 2) {
            let raycast = get_raycast(&scene, x, y);
            let color = compute_shading(&scene, raycast);
            scene.canvas.write_pixel(x, y, color);
        }
    }
    scene.canvas.save("test.png")
}

fn get_raycast(scene: &Scene, x: i32, y: i32) -> Option<Raycast> {
    let origin = scene.origin;
    let direction = Vector3::new(
        x as f64 * scene.viewport.x / scene.canvas.width as f64,
        y as f64 * scene.viewport.y / scene.canvas.height as f64,
        scene.viewport.z,
    );

    Raycast::compute(&scene, origin, direction, (scene.viewport.z, f64::INFINITY))
}
