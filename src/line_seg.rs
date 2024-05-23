use crate::matrix::{Matrix21, Matrix22};
use crate::vec2::Vec2;

pub struct LineSeg {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl LineSeg {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    pub fn len(&self) -> f64 {
        (&self.p2 - &self.p1).len()
    }

    pub fn x_diff(&self) -> f64 {
        self.p2.x - self.p1.x
    }

    pub fn y_diff(&self) -> f64 {
        self.p2.y - self.p1.y
    }

    pub fn intersects(&self, other: &LineSeg) -> Option<Vec2> {
        // Parametric equations
        // (x(s), y(s)) = (p1.x + s(p2.x-p1.x), p1.y + s(p2.y-p1.y))
        // The point at s along the line

        // L1(s) = L2(t)
        // s(p2.x-p1.x) - t(p2.x, p1.x) = other.p1.x - self.p1.x
        // s(p2.y-p1.y) - t(p2.y, p1.y) = other.p1.y - self.p1.y

        //                       A                            X                B
        // ( self.p2.x-self.p1.x, -(other.p2.x-other.p1.x) ) (s) = ( other.p1.x-self.p1.x )
        // ( self.p2.y-self.p1.y, -(other.p2.y-other.p1.y) ) (t) = ( other.p1.y-self.p1.y )
        // ^ AX = B
        // X = A^-1 B

        let a = Matrix22::new(
            self.p2.x - self.p1.x,
            -(other.p2.x - other.p1.x),
            self.p2.y - self.p1.y,
            -(other.p2.y - other.p1.y),
        );

        let b = Matrix21::new(other.p1.x - self.p1.x, other.p1.y - self.p1.y);

        let x = &a.inverse() * &b;

        if (0.0..=1.0).contains(&x.data[0]) && (0.0..=1.0).contains(&x.data[1]) {
            Some(Vec2::new(
                self.p1.x + x.data[0] * (self.p2.x - self.p1.x),
                self.p1.y + x.data[0] * (self.p2.y - self.p1.y),
            ))
        } else {
            None
        }
    }
}
