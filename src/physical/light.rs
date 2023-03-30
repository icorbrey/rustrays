use crate::math::{raycast::Raycast, vector3::Vector3};

use super::scene::Scene;

#[derive(Copy, Clone)]
pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, origin: Vector3 },
    Directional { intensity: f64, direction: Vector3 },
}

pub fn compute_is_occluded(scene: &Scene, light: Light, position: Vector3) -> bool {
    match light {
        Light::Ambient { .. } => false,
        Light::Point { origin, .. } => {
            match Raycast::compute(scene, position, origin - position, (0.001, f64::INFINITY)) {
                Raycast::Intersection { .. } => true,
                Raycast::NoIntersection => false,
            }
        }
        Light::Directional { direction, .. } => {
            match Raycast::compute(scene, position, direction, (0.001, f64::INFINITY)) {
                Raycast::Intersection { .. } => true,
                Raycast::NoIntersection => false,
            }
        }
    }
}

pub fn compute_diffusion(light: Light, position: Vector3, normal: Vector3) -> f64 {
    match light {
        Light::Ambient { intensity } => intensity,
        Light::Point { intensity, origin } => {
            let direction = origin - position;

            // Don't shine light on surfaces pointing away from the light
            if normal.dot(direction) < 0.0 {
                return 0.0;
            }

            // Light contribution is the highest when facing the light source
            intensity * direction.angle_from(normal).cos()
        }
        Light::Directional {
            intensity,
            direction,
        } => {
            // Don't shine light on surfaces pointing away from the light
            if normal.dot(direction) < 0.0 {
                return 0.0;
            }

            // Light contribution is the highest when facing the light source
            intensity * direction.angle_from(normal).cos()
        }
    }
}
