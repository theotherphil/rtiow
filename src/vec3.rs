
use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    Sub
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e:  [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3::new(
        lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
        -(lhs.e[0] * rhs.e[2] - lhs.e[2] * rhs.e[0]),
        lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0]
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

macro_rules! impl_vec_vec_op {
    ($op:ident, $op_name:ident) => {
        impl $op for Vec3 {
            type Output = Vec3;

            fn $op_name(self, rhs: Self) -> Self {
                Vec3::new(
                    $op::$op_name(self.e[0], rhs.e[0]),
                    $op::$op_name(self.e[1], rhs.e[1]),
                    $op::$op_name(self.e[2], rhs.e[2])
                )
            }
        }
    }
}

impl_vec_vec_op!(Add, add);
impl_vec_vec_op!(Sub, sub);
impl_vec_vec_op!(Mul, mul);

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs.mul(self)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}


impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}
