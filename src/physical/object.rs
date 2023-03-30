use crate::math::vector3::Vector3;

use super::shader::Shader;

#[derive(Copy, Clone)]
pub enum Object {
    Sphere {
        position: Vector3,
        shader: Shader,
        radius: f64,
    },
}

impl Object {
    pub fn compute_intersections(self, origin: Vector3, direction: Vector3) -> Option<(f64, f64)> {
        match self {
            Object::Sphere {
                position, radius, ..
            } => {
                let offset = origin - position;

                let a = direction.dot(direction);
                let b = 2.0 * offset.dot(direction);
                let c = offset.dot(offset) - radius.powf(2.0);

                let d = b.powf(2.0) - 4.0 * a * c;

                // Ray does not intersect sphere if discriminant is negative
                if d < 0.0 {
                    return None;
                }

                Some(((-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a)))
            }
        }
    }

    pub fn compute_normal(self, point: Vector3) -> Vector3 {
        match self {
            Object::Sphere { position, .. } => {
                // Normal vector always faces directly away from the center
                (point - position).normalized()
            }
        }
    }

    pub fn get_shader(self) -> Shader {
        match self {
            Object::Sphere { shader, .. } => shader,
        }
    }
}
