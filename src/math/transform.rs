use crate::math::Vector3;

#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vector3,
}

impl Transform {
    pub fn new(position: Vector3) -> Self {
        Transform { position }
    }
}
