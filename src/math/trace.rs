use crate::canvas::Canvas;
use crate::light::Light;
use crate::object::Object;
use crate::shader::compute_shading;

use super::{Color, Ray, Vector3};

pub fn get_raycast_direction(x: i32, y: i32, viewport_size: Vector3, canvas: &Canvas) -> Vector3 {
    let (width, height) = canvas.size;
    Vector3::new(
        x as f64 * viewport_size.x / width as f64,
        y as f64 * viewport_size.y / height as f64,
        viewport_size.z,
    )
}

pub fn trace_ray(objects: Vec<Object>, lights: Vec<Light>, ray: Ray, range: (f64, f64)) -> Color {
    let (object, t) = compute_closest_object(range, objects, ray);
    match object {
        Some(object) => compute_shading(object, lights, ray, t),
        None => super::Color::new(255, 255, 255),
    }
}

pub fn compute_closest_object(
    range: (f64, f64),
    objects: Vec<Object>,
    ray: Ray,
) -> (Option<Object>, f64) {
    let (t_min, t_max) = range;
    let mut target: (Option<Object>, f64) = (None, f64::INFINITY);

    for object in objects {
        let (t1, t2) = object.compute_intersections(ray);

        if t_min < t1 && t1 < t_max && t1 < target.1 {
            target = (Some(object), t1);
        }

        if t_min < t2 && t2 < t_max && t2 < target.1 {
            target = (Some(object), t2);
        }
    }

    target
}
