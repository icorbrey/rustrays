use crate::math::vector3::Vector3;

#[derive(Copy, Clone)]
pub enum Light {
    Ambient(f64),
    Point(f64, Vector3),
    Directional(f64, Vector3),
}

pub fn compute_light_contribution(light: Light, position: Vector3, normal: Vector3) -> f64 {
    match light {
        Light::Ambient(intensity) => intensity,
        Light::Point(intensity, origin) => {
            let offset = origin - position;
            if normal.dot(offset) > 0.0 {
                intensity * normal.dot(offset) / (normal.magnitude() * offset.magnitude())
            } else {
                0.0
            }
        }
        Light::Directional(intensity, direction) => {
            if normal.dot(direction) > 0.0 {
                intensity * normal.dot(direction) / (normal.magnitude() * direction.magnitude())
            } else {
                0.0
            }
        }
    }
}
