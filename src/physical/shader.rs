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

pub fn compute_shading(scene: &Scene, raycast: Raycast) -> Color {
    match raycast {
        Raycast::Intersection { object, point, .. } => match object.get_shader() {
            Shader::Lit { color, .. } => {
                let normal = object.compute_normal(point);

                let mut illumination = 0.0;

                for light in &scene.lights {
                    if compute_is_occluded(scene, *light, point) {
                        continue;
                    }

                    illumination += compute_diffusion(*light, point, normal);
                }

                color * illumination
            }
        },
        Raycast::NoIntersection => Color::new(255, 255, 255),
    }
}
