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
            let direction = origin - position;

            // Don't shine light on surfaces pointing away from the light
            if normal.dot(direction) < 0.0 {
                return 0.0;
            }

            // Light contribution is the highest when facing the light source
            intensity * direction.angle_from(normal).cos()
        }
        Light::Directional(intensity, direction) => {
            // Don't shine light on surfaces pointing away from the light
            if normal.dot(direction) < 0.0 {
                return 0.0;
            }

            // Light contribution is the highest when facing the light source
            intensity * direction.angle_from(normal).cos()
        }
    }
}
