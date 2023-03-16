use crate::{
    light::{compute_light_contribution, Light},
    math::{Color, Ray},
    object::Object,
};

#[derive(Copy, Clone)]
pub enum Shader {
    Lit(Color),
}

pub fn compute_shading(object: Object, lights: Vec<Light>, ray: Ray, t: f64) -> Color {
    match object.get_shader() {
        Shader::Lit(color) => {
            let point = object.get_intersection(ray, t);
            let normal = object.compute_normal(ray, t);
            let illumination = lights
                .into_iter()
                .map(|light| compute_light_contribution(light, point, normal))
                .reduce(|x, y| x + y)
                .unwrap_or(1.0);
            color * illumination
        }
    }
}
