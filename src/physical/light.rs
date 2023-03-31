use crate::math::{raycast::Raycast, vector3::Vector3};

use super::scene::Scene;

#[derive(Copy, Clone)]
pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, origin: Vector3 },
    Directional { intensity: f64, direction: Vector3 },
}

pub fn compute_is_occluded(scene: &Scene, light: Light, raycast: Raycast) -> bool {
    let light_direction = match light {
        Light::Point { origin, .. } => origin - raycast.point,
        Light::Directional { direction, .. } => direction,
        Light::Ambient { .. } => return false,
    };

    Raycast::compute(
        scene,
        raycast.point,
        light_direction,
        (0.001, f64::INFINITY),
    )
    .is_some()
}

pub fn compute_diffusion(light: Light, raycast: Raycast) -> f64 {
    let (intensity, light_direction) = match light {
        Light::Point { intensity, origin } => (intensity, origin - raycast.point),
        Light::Directional {
            intensity,
            direction,
        } => (intensity, direction),
        Light::Ambient { intensity } => return intensity,
    };

    // Don't shine light on surfaces pointing away from the light
    if raycast.normal.dot(light_direction) < 0.0 {
        return 0.0;
    }

    // Light contribution is the highest when facing the light source
    intensity * light_direction.angle_from(raycast.normal).cos()
}
