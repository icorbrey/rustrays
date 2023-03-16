use image::ImageResult;

use crate::{
    math::{color::Color, ray::Ray, vector3::Vector3},
    object::Object,
    scene::Scene,
    shader::compute_shading,
};

pub fn render(mut scene: Scene) -> ImageResult<()> {
    let (width, height) = scene.canvas.size;
    for x in (-1 * width as i32 / 2)..(width as i32 / 2) {
        for y in (-1 * height as i32 / 2)..(height as i32 / 2) {
            let direction = get_raycast(&scene, x, y);
            let color = trace_ray(&scene, direction);
            scene.canvas.write_pixel(x, y, color);
        }
    }
    scene.canvas.save("test.png")
}

fn get_raycast(scene: &Scene, x: i32, y: i32) -> Ray {
    let (width, height) = scene.canvas.size;
    Ray::new(
        scene.origin,
        Vector3::new(
            x as f64 * scene.viewport.x / width as f64,
            y as f64 * scene.viewport.y / height as f64,
            scene.viewport.z,
        ),
    )
}

fn trace_ray(scene: &Scene, ray: Ray) -> Color {
    let (object, t) = compute_closest_object(scene, ray);
    match object {
        Some(object) => compute_shading(scene, object, ray, t),
        None => Color::new(255, 255, 255),
    }
}

pub fn compute_closest_object(scene: &Scene, ray: Ray) -> (Option<Object>, f64) {
    let (t_min, t_max) = (scene.viewport.z, f64::INFINITY);
    let mut target: (Option<Object>, f64) = (None, f64::INFINITY);

    for object in &scene.objects {
        let (t1, t2) = object.compute_intersections(ray);

        if t_min < t1 && t1 < t_max && t1 < target.1 {
            target = (Some(*object), t1);
        }

        if t_min < t2 && t2 < t_max && t2 < target.1 {
            target = (Some(*object), t2);
        }
    }

    target
}
