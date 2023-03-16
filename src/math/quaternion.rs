use std::f64::consts::PI;
use std::fmt::Display;

use num::traits::Pow;
use num::NumCast;

use super::Vector3;

#[derive(Copy, Clone)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: impl NumCast, x: impl NumCast, y: impl NumCast, z: impl NumCast) -> Self {
        Quaternion {
            w: w.to_f64().unwrap(),
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
            z: z.to_f64().unwrap(),
        }
    }

    pub fn to_euler_angles(self) -> Vector3 {
        let sin_r_cos_p = 2.0 * (self.w * self.x + self.y * self.z);
        let cos_r_cos_p = 1.0 - 2.0 * (self.x.pow(2.0) + self.y.pow(2.0));
        let sin_p = (1.0 + 2.0 * (self.w * self.y - self.x * self.z)).sqrt();
        let cos_p = (1.0 - 2.0 * (self.w * self.y - self.x * self.z)).sqrt();
        let sin_y_cos_p = 2.0 * (self.w * self.z + self.x * self.y);
        let cos_y_cos_p = 1.0 - 2.0 * (self.y.pow(2.0) + self.z.pow(2.0));

        Vector3::new(
            sin_r_cos_p.atan2(cos_r_cos_p),
            2.0 * sin_p.atan2(cos_p) - PI / 2.0,
            sin_y_cos_p.atan2(cos_y_cos_p),
        )
    }

    pub fn from_euler_angles(euler_angles: Vector3) -> Quaternion {
        let (roll, pitch, yaw) = (euler_angles.x, euler_angles.y, euler_angles.z);

        let cos_r = (roll / 2.0).cos();
        let sin_r = (roll / 2.0).sin();
        let cos_p = (pitch / 2.0).cos();
        let sin_p = (pitch / 2.0).sin();
        let cos_y = (yaw / 2.0).cos();
        let sin_y = (yaw / 2.0).sin();

        Quaternion::new(
            cos_r * cos_p * cos_y + sin_r * sin_p * sin_y,
            sin_r * cos_p * cos_y - cos_r * sin_p * sin_y,
            cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
            cos_r * cos_p * sin_y - sin_r * sin_p * cos_y,
        )
    }

    pub fn forward() -> Self {
        Self::from_euler_angles(Vector3::new(0, 0, 1))
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion ⟨{}, {}, {}, {}⟩",
            self.w, self.x, self.y, self.z
        )
    }
}
