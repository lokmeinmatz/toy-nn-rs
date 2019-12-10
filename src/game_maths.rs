use std::ops;
use ggez::mint::Point2;
use std::ops::{Add, Mul, MulAssign, AddAssign, Sub};
use std::fmt::{Debug, Formatter, Error, Write};


#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn unit_from_angle(angle: f32) -> Self {
        Vec2 {
            x: angle.cos(),
            y: angle.sin()
        }
    }

    pub const fn zero() -> Self { Vec2::new(0.0, 0.0) }
}



impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Vec2(x: {} y: {})", self.x, self.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(a: (f32, f32)) -> Self {
        Vec2 { x: a.0, y: a.1 }
    }
}

impl Into<Point2<f32>> for Vec2 {
    fn into(self) -> Point2<f32> {
        [self.x, self.y].into()
    }
}


pub fn line_line_intersection((p1, p2): (Vec2, Vec2), (p3, p4) : (Vec2, Vec2)) -> Option<Vec2> {

    let t_dividend = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let u_dividend = (p1.x - p2.x) * (p1.y - p3.y) - (p1.y - p2.y) * (p1.x - p3.x);

    let divisor = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);

    if divisor.abs() < 0.001 {
        return None;
    }
    let t = t_dividend / divisor;
    if t < 0.0 || t > 1.0 {
        return None;
    }

    let u = u_dividend / divisor;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let d = p2 - p1;
    return Some(p1 + d * t);
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
        assert!(i < 3 && j < 3);
        self.0[i][j]
    }

    pub fn translate(translate: Vec2) -> Mat3 {
        Mat3([
            [1.0, 0.0, translate.x],
            [0.0, 1.0, translate.y],
            [0.0, 0.0, 1.0]
        ])
    }

    pub fn rotate(rad: f32) -> Mat3 {
        let cos = rad.cos();
        let sin = rad.sin();
        Mat3([
            [cos, -sin, 0.0],
            [sin, cos, 0.0],
            [0.0, 0.0, 1.0]
        ])
    }

    pub fn scale(scale: Vec2) -> Mat3 {
        Mat3([
            [scale.x, 0.0, 0.0],
            [0.0, scale.y, 0.0],
            [0.0, 0.0, 0.0]
        ])
    }

    pub fn affine(translate: Vec2, rotate: f32, scale: Vec2) -> Mat3 {
        let cos_rot = rotate.cos();
        let sin_rot = rotate.sin();

        Mat3([
            [scale.x * cos_rot, -scale.y * sin_rot, translate.x],
            [scale.x * sin_rot, scale.y * cos_rot , translate.y],
            [0.0              , 0.0               , 1.0],
        ])
    }
}

impl ops::Mul<Mat3> for Mat3 {
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

impl Debug for Mat3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Mat3{{\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}}}", self.0[0][0], self.0[0][1], self.0[0][2], self.0[1][0], self.0[1][1], self.0[1][2], self.0[2][0], self.0[2][1], self.0[2][2])
    }
}

impl ops::Mul<Vec2> for Mat3 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.get(0, 0) * rhs.x + self.get(0, 1) * rhs.y + self.get(0, 2),
            y: self.get(1, 0) * rhs.x + self.get(1, 1) * rhs.y + self.get(1, 2)
        }
    }
}

#[test]
fn test_maths() {
    let vup = Vec2::new(0.0, 1.0);

    let transform = Mat3::rotate(90f32.to_radians());

    let vleft = transform * vup;
    println!("{:?}", vleft);
    assert!(vleft.x == -1.0 && vleft.y.abs() <= 0.001);


    // test affine
    let transform2 = Mat3::affine(Vec2::new(1.0, 0.0), 0f32.to_radians(), Vec2::new(1.0, 1.0));
    println!("{:?}", transform2);
    let res2 = transform2 * vup;

    println!("{:?}", res2);
}