use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_arr(&self) -> [i32; 2] {
        [self.x, self.y]
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Clone for Vec2i {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<[i32; 2]> for Vec2i {
    fn from(value: [i32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Vec2i> for [i32; 2] {
    fn from(value: Vec2i) -> Self {
        [value.x, value.y]
    }
}

impl ops::Add<&Vec2i> for &Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: &Vec2i) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<&Vec2i> for Vec2i {
    fn add_assign(&mut self, rhs: &Vec2i) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<&Vec2i> for &Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: &Vec2i) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<&Vec2i> for Vec2i {
    fn sub_assign(&mut self, rhs: &Vec2i) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<i32> for &Vec2i {
    type Output = Vec2i;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<&Vec2i> for Vec2i {
    fn mul_assign(&mut self, rhs: &Vec2i) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Div<i32> for &Vec2i {
    type Output = Vec2i;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<&Vec2i> for Vec2i {
    fn div_assign(&mut self, rhs: &Vec2i) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
