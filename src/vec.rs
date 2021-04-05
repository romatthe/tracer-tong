use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Mul;

pub type Point = Vec3;

pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn empty() -> Self {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(&self) -> f32 {
        (self[0] * self[0]) + (self[1] * self[1]) + (self[2] * self[2])
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
    }

    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ]
        }
    }

    #[inline]
    pub fn unit(self) -> Vec3 {
        let len = self.len();

        self / len
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()]
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
        }
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs[0];
        self.e[1] += rhs[2];
        self.e[2] += rhs[3];
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs[0];
        self.e[1] -= rhs[2];
        self.e[2] -= rhs[3];
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs]
        }
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs]
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
        }
    }
}

impl std::ops::MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs[0];
        self.e[1] *= rhs[1];
        self.e[2] *= rhs[2];
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs]
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}