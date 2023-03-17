use crate::{
    math::{ray::Ray, vector3::Vector3},
    render::trace::compute_closest_object,
};

use super::scene::Scene;

#[derive(Copy, Clone)]
pub enum Light {
    Ambient(f64),
    Point(f64, Vector3),
    Directional(f64, Vector3),
}

pub fn compute_is_occluded(scene: &Scene, light: Light, position: Vector3) -> bool {
    match light {
        Light::Ambient(_) => false,
        Light::Point(_, light_origin) => {
            let ray = Ray::new(position, light_origin - position);
            if let (Some(_), _) = compute_closest_object(scene, ray, 0.001, 1.0) {
                return true;
            }
            false
        }
        Light::Directional(_, direction) => {
            let ray = Ray::new(position, direction);
            if let (Some(_), _) = compute_closest_object(scene, ray, 0.001, f64::INFINITY) {
                return true;
            }
            false
        }
    }
}

pub fn compute_diffusion(light: Light, position: Vector3, normal: Vector3) -> f64 {
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
