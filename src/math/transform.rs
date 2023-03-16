use crate::math::{Quaternion, Vector3};

#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn new(position: Vector3, rotation: Quaternion) -> Self {
        Transform { position, rotation }
    }

    pub fn from_position(position: Vector3) -> Self {
        Self::new(position, Quaternion::forward())
    }
}
