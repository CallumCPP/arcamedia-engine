use std::ops;

pub struct Matrix22 {
    pub data: [[f64; 2]; 2],
}

pub struct Matrix21 {
    pub data: [f64; 2],
}

impl Matrix22 {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            data: [[a, b], [c, d]],
        }
    }

    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    pub fn inverse(&self) -> Self {
        let recip_determinant = 1.0 / self.determinant();

        Self {
            data: [
                [
                    recip_determinant * (self.data[1][1]),
                    recip_determinant * (-self.data[0][1]),
                ],
                [
                    recip_determinant * (-self.data[1][0]),
                    recip_determinant * (self.data[0][0]),
                ],
            ],
        }
    }
}

impl ops::Mul<&Matrix21> for &Matrix22 {
    type Output = Matrix21;

    fn mul(self, rhs: &Matrix21) -> Self::Output {
        Self::Output {
            data: [
                self.data[0][0] * rhs.data[0] + self.data[0][1] * rhs.data[1],
                self.data[1][0] * rhs.data[0] + self.data[1][1] * rhs.data[1],
            ],
        }
    }
}

impl Matrix21 {
    pub fn new(a: f64, b: f64) -> Self {
        Self { data: [a, b] }
    }
}
