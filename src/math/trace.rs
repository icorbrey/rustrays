use crate::canvas::Canvas;
use crate::light::{compute_light_contribution, Light};
use crate::object::Object;

use super::{Color, Ray, Vector3};

pub fn get_raycast_direction(x: i32, y: i32, viewport_size: Vector3, canvas: &Canvas) -> Vector3 {
    let (width, height) = canvas.size;
    Vector3::new(
        x as f64 * viewport_size.x / width as f64,
        y as f64 * viewport_size.y / height as f64,
        viewport_size.z,
    )
}

pub fn trace_ray(objects: &Vec<Object>, lights: &Vec<Light>, ray: Ray, range: (f64, f64)) -> Color {
    let (object, t) = compute_closest_object(range, objects, ray);
    match object {
        Some(object) => {
            object.get_shader().get_color() * compute_illumination(object, ray, t, lights)
        }
        None => super::Color::new(255, 255, 255),
    }
}

pub fn compute_illumination(object: Object, ray: Ray, t: f64, lights: &Vec<Light>) -> f64 {
    let point = object.get_intersection(ray, t);
    let normal = object.compute_normal(ray, t);
    lights
        .into_iter()
        .map(|light| compute_light_contribution(*light, point, normal))
        .reduce(|x, y| x + y)
        .unwrap_or(1.0)
}

pub fn compute_closest_object(
    range: (f64, f64),
    objects: &Vec<Object>,
    ray: Ray,
) -> (Option<Object>, f64) {
    let (t_min, t_max) = range;
    let mut target: (Option<Object>, f64) = (None, f64::INFINITY);

    for object in objects {
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
