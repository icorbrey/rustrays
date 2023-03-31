use crate::math::{color::Color, raycast::Raycast};

use super::{
    light::{compute_diffusion, compute_is_occluded, compute_specular_reflection},
    scene::Scene,
};

#[derive(Copy, Clone)]
pub enum Shader {
    Lit {
        specular_reflection: Option<f64>,
        reflectivity: f64,
        color: Color,
    },
}

pub fn compute_shading(scene: &Scene, raycast: Option<Raycast>) -> Color {
    match raycast {
        Some(raycast) => match raycast.object.get_shader() {
            Shader::Lit {
                color,
                specular_reflection,
                ..
            } => {
                let mut illumination = 0.0;

                for light in &scene.lights {
                    if compute_is_occluded(scene, *light, raycast) {
                        continue;
                    }

                    illumination += compute_diffusion(*light, raycast);

                    if let Some(s) = specular_reflection {
                        illumination += compute_specular_reflection(*light, raycast, s);
                    }
                }

                color * illumination
            }
        },
        None => Color::new(255, 255, 255),
    }
}
