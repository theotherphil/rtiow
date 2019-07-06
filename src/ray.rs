
use crate::vec3::*;

/// A + t * B
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
}
