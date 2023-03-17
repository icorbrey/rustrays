use crate::math::{color::Color, ray::Ray};

use super::{
    light::{compute_diffusion, compute_is_occluded},
    object::Object,
    scene::Scene,
};

#[derive(Copy, Clone)]
pub enum Shader {
    Lit(VisualProperties),
}

#[derive(Copy, Clone)]
pub struct VisualProperties {
    pub color: Color,
    pub specular_reflection: Option<f64>,
}

pub fn compute_shading(scene: &Scene, object: Object, ray: Ray, t: f64) -> Color {
    match object.get_shader() {
        Shader::Lit(properties) => {
            let position = ray.get_point(t);
            let normal = object.compute_normal(ray, t);

            let mut illumination = 0.0;

            for light in &scene.lights {
                if compute_is_occluded(scene, *light, position) {
                    continue;
                }

                illumination += compute_diffusion(*light, position, normal);
            }

            properties.color * illumination
        }
    }
}
