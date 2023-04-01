use crate::math::{raycast::Raycast, vector3::Vector3};

use super::scene::Scene;

#[derive(Copy, Clone)]
pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, origin: Vector3 },
    Directional { intensity: f64, direction: Vector3 },
}

pub fn compute_is_occluded(scene: &Scene, light: Light, raycast: Raycast) -> bool {
    if let (_, Some(direction)) = get_light_details(light, raycast) {
        Raycast::compute(scene, raycast.point, direction, (0.001, f64::INFINITY)).is_some()
    } else {
        false
    }
}

pub fn compute_diffusion(light: Light, raycast: Raycast) -> f64 {
    let (intensity, direction) = get_light_details(light, raycast);

    if let Some(direction) = direction {
        // Don't shine light on surfaces pointing away from the light
        if raycast.normal.dot(direction) < 0.0 {
            return 0.0;
        }

        // Light contribution is the highest when facing the light source
        intensity * direction.angle_from(raycast.normal).cos()
    } else {
        intensity
    }
}

pub fn compute_specular_reflection(
    light: Light,
    raycast: Raycast,
    specular_reflection: f64,
) -> f64 {
    let (intensity, direction) = get_light_details(light, raycast);

    if let Some(direction) = direction {
        let reflection = get_reflection(raycast.normal, direction);

        // Don't reflect off of surfaces pointing away from the light
        if reflection.dot(raycast.view) < 0.0 {
            return 0.0;
        }

        // Light contribution is the highest when the light reflects directly into the camera
        intensity
            * reflection
                .angle_from(raycast.view)
                .cos()
                .powf(specular_reflection)
    } else {
        0.0
    }
}

pub fn get_light_details(light: Light, raycast: Raycast) -> (f64, Option<Vector3>) {
    match light {
        Light::Ambient { intensity } => (intensity, None),
        Light::Point { intensity, origin } => (intensity, Some(origin - raycast.point)),
        Light::Directional {
            intensity,
            direction,
        } => (intensity, Some(direction)),
    }
}

pub fn get_reflection(normal: Vector3, direction: Vector3) -> Vector3 {
    normal * normal.dot(direction) * 2 - direction
}
