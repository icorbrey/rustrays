use std::fmt::Display;
use std::ops;

use num::NumCast;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: impl NumCast, y: impl NumCast, z: impl NumCast) -> Self {
        Vector3 {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
            z: z.to_f64().unwrap(),
        }
    }

    pub fn angle_from(self, rhs: Self) -> f64 {
        (self.dot(rhs) / (self.magnitude() * rhs.magnitude())).acos()
    }

    pub fn cross(self, rhs: Self) -> Self {
        self.angle_from(rhs).sin()
            * self.magnitude()
            * rhs.magnitude()
            * Vector3::new(
                self.y * rhs.z - self.z * rhs.y,
                self.z * rhs.x - self.x * rhs.z,
                self.x * rhs.y - self.y * rhs.x,
            )
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0))
    }

    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    pub fn zero() -> Self {
        Vector3::new(0, 0, 0)
    }
}

impl ops::Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<N> ops::Mul<N> for Vector3
where
    N: NumCast,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        let rhs = rhs.to_f64().unwrap();
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl<N> ops::Div<N> for Vector3
where
    N: NumCast,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        let rhs = rhs.to_f64().unwrap();
        assert_ne!(rhs, num::zero());
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3 ⟨{}, {}, {}⟩", self.x, self.y, self.z)
    }
}
