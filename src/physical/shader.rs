use crate::math::{color::Color, ray::Ray};

use super::{light::compute_light_contribution, object::Object, scene::Scene};

#[derive(Copy, Clone)]
pub enum Shader {
    Lit(Color),
}

pub fn compute_shading(scene: &Scene, object: Object, ray: Ray, t: f64) -> Color {
    match object.get_shader() {
        Shader::Lit(color) => {
            let position = ray.get_point(t);
            let normal = object.compute_normal(ray, t);
            let illumination = (scene.lights.clone().into_iter())
                .map(|light| compute_light_contribution(light, position, normal))
                .reduce(|x, y| x + y)
                .unwrap_or(1.0);
            color * illumination
        }
    }
}
