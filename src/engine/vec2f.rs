use js_sys::Math::{cos, sin, sqrt};
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn as_arr(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn normalize(&self) -> Self {
        let len = self.len();

        if len == 0.0 {
            return Self { x: 0.0, y: 0.0 };
        }

        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn len(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn dot(&self, other: &Vec2f) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn rotated(&mut self, angle: f64) -> Self {
        Self {
            x: self.x * cos(angle) - self.y * sin(angle),
            y: self.y * cos(angle) + self.x * sin(angle),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Clone for Vec2f {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<[f64; 2]> for Vec2f {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Vec2f> for [f64; 2] {
    fn from(value: Vec2f) -> Self {
        [value.x, value.y]
    }
}

impl ops::Add<&Vec2f> for &Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: &Vec2f) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<&Vec2f> for Vec2f {
    fn add_assign(&mut self, rhs: &Vec2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<&Vec2f> for &Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: &Vec2f) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<&Vec2f> for Vec2f {
    fn sub_assign(&mut self, rhs: &Vec2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f64> for &Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<&Vec2f> for Vec2f {
    fn mul_assign(&mut self, rhs: &Vec2f) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Div<f64> for &Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<&Vec2f> for Vec2f {
    fn div_assign(&mut self, rhs: &Vec2f) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
