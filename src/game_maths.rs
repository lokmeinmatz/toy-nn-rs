use std::ops;
use std::ops::Mul;

pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat3) -> Self::Output {
        Vec2 {
            x: self.x * rhs.get(0, 0) + self.y
        }
    }
}

pub struct Mat3 ( [[f32; 3]; 3] );

impl Mat3 {
    pub fn zero() -> Self {
        Mat3 ([[0.0; 3]; 3])
    }

    pub fn identity() -> Self {
        Mat3(
            [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        )
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        assert!(i < 4 && j < 4);
        self.0[i][j]
    }
}

impl ops::Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Mat3::zero();

        for i in 0..4 {
            for j in 0..4 {
                res.0[i][j] =   self.0[i][0] * rhs.0[0][j] +
                                self.0[i][1] * rhs.0[1][j] +
                                self.0[i][2] * rhs.0[2][j] +
                                self.0[i][3] * rhs.0[3][j];
            }
        }
        res
    }
}