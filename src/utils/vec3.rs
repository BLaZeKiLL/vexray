use core::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color3 = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Vec3 {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.sqr_magnitude())
    }

    pub fn sqr_magnitude(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    /// Returns a unit vector
    pub fn normalized(&self) -> Vec3 {
        let m = self.magnitude();
        
        Vec3 { e: [self.x() / m, self.y() / m, self.z() / m] }
    }

	pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
		a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
	}

	pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
		Vec3 { e: [
			a.y() * b.z() - a.z() * b.y(),
			a.z() * b.x() - a.x() * b.z(),
			a.x() * b.y() - a.y() * b.x()
		] }
	}

    pub fn write_color(&self) -> String {
        format!("{} {} {} \n", 
            (255.999 * self.x()).floor() as i64, 
            (255.999 * self.y()).floor() as i64,
            (255.999 * self.z()).floor() as i64
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.x(), -self.y(), -self.z()]}
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()] }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()] }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()] }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [self * rhs.x(), self * rhs.y(), self * rhs.z()] }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
        self.e[2] /= rhs.z();
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X = {}, Y = {}, Z = {})", self.x(), self.y(), self.z())
    }
}