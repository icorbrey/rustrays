use crate::math::{Ray, Transform, Vector3};
use crate::shader::Shader;

#[derive(Copy, Clone)]
pub enum Object {
    Sphere(Transform, f64, Shader),
}

impl Object {
    pub fn compute_intersections(self, ray: Ray) -> (f64, f64) {
        match self {
            Object::Sphere(transform, radius, _) => {
                let offset = ray.origin - transform.position;

                let a = ray.direction.dot(ray.direction);
                let b = 2.0 * offset.dot(ray.direction);
                let c = offset.dot(offset) - radius.powf(2.0);

                let d = b.powf(2.0) - 4.0 * a * c;

                if d >= 0.0 {
                    ((-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a))
                } else {
                    (f64::INFINITY, f64::INFINITY)
                }
            }
        }
    }

    pub fn get_intersection(self, ray: Ray, t: f64) -> Vector3 {
        ray.origin + ray.direction * t
    }

    pub fn compute_normal(self, ray: Ray, t: f64) -> Vector3 {
        match self {
            Object::Sphere(transform, _, _) => {
                (self.get_intersection(ray, t) - transform.position).normalized()
            }
        }
    }

    pub fn get_shader(self) -> Shader {
        match self {
            Object::Sphere(_, _, shader) => shader,
        }
    }
}
