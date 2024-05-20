use js_sys::Math::sqrt;
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn as_arr(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    pub fn normalize(&self) -> Self {
        let len = self.len();

        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn len(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn dot_static(first: &Vec2, second: &Vec2) -> f64 {
        first.x * second.x + first.y * second.y
    }
}

impl Clone for Vec2 {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<[f64; 2]> for Vec2 {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Vec2> for [f64; 2] {
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
