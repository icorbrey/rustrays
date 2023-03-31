use crate::math::{color::Color, raycast::Raycast};

use super::{
    light::{compute_diffusion, compute_is_occluded},
    scene::Scene,
};

#[derive(Copy, Clone)]
pub enum Shader {
    Lit {
        specular_reflection: Option<f64>,
        color: Color,
    },
}

#[derive(Copy, Clone)]
pub struct VisualProperties {
    pub color: Color,
    pub specular_reflection: Option<f64>,
}

pub fn compute_shading(scene: &Scene, raycast: Option<Raycast>) -> Color {
    match raycast {
        Some(raycast) => match raycast.object.get_shader() {
            Shader::Lit { color, .. } => {
                let mut illumination = 0.0;

                for light in &scene.lights {
                    if compute_is_occluded(scene, *light, raycast) {
                        continue;
                    }

                    illumination += compute_diffusion(*light, raycast);
                }

                color * illumination
            }
        },
        None => Color::new(255, 255, 255),
    }
}
