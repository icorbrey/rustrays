use crate::math::{color::Color, raycast::Raycast};

use super::{
    light::{compute_diffusion, compute_is_occluded, compute_specular_reflection},
    scene::Scene,
};

#[derive(Copy, Clone)]
pub struct Shader {
    pub specular_reflection: Option<f64>,
    pub reflectivity: Option<f64>,
    pub color: Color,
}

pub fn compute_shading(scene: &Scene, raycast: Raycast) -> Color {
    let shader = raycast.object.get_shader();
    let mut illumination = 0.0;

    for light in &scene.lights {
        if compute_is_occluded(scene, *light, raycast) {
            continue;
        }

        illumination += compute_diffusion(*light, raycast);

        if let Some(s) = shader.specular_reflection {
            illumination += compute_specular_reflection(*light, raycast, s);
        }
    }

    shader.color * illumination
}
