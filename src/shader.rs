use crate::math::Color;

#[derive(Copy, Clone)]
pub enum Shader {
    Lit(Color),
}

impl Shader {
    pub fn get_color(self) -> Color {
        match self {
            Shader::Lit(color) => color,
        }
    }
}
