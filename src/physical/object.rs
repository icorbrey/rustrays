use crate::math::{ray::Ray, vector3::Vector3};

use super::shader::Shader;

#[derive(Copy, Clone)]
pub enum Object {
    Sphere(Vector3, f64, Shader),
}

impl Object {
    pub fn compute_intersections(self, ray: Ray) -> (f64, f64) {
        match self {
            Object::Sphere(position, radius, _) => {
                let offset = ray.origin - position;

                let a = ray.direction.dot(ray.direction);
                let b = 2.0 * offset.dot(ray.direction);
                let c = offset.dot(offset) - radius.powf(2.0);

                let d = b.powf(2.0) - 4.0 * a * c;

                // Ray does not intersect sphere if discriminant is negative
                if d < 0.0 {
                    return (f64::INFINITY, f64::INFINITY);
                }

                ((-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a))
            }
        }
    }

    pub fn compute_normal(self, ray: Ray, t: f64) -> Vector3 {
        match self {
            Object::Sphere(position, _, _) => {
                // Normal vector always faces directly away from the center
                (ray.get_point(t) - position).normalized()
            }
        }
    }

    pub fn get_shader(self) -> Shader {
        match self {
            Object::Sphere(_, _, shader) => shader,
        }
    }
}
