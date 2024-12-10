use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{aabb::Axis, interval::Interval, random_0_1, random_range};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn x(self) -> f64 {
        self.x
    }

    pub const fn y(self) -> f64 {
        self.y
    }

    pub const fn z(self) -> f64 {
        self.z
    }

    pub const fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn random() -> Self {
        Vec3::new(random_0_1(), random_0_1(), random_0_1())
    }

    pub const fn scale(self, scale: Vec3) -> Self {
        Vec3::new(self.x * scale.x, self.y * scale.y, self.z * scale.z)
    }

    pub fn random_range(interval: Interval) -> Self {
        Vec3::new(
            random_range(interval),
            random_range(interval),
            random_range(interval),
        )
    }

    pub const fn lenght_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn lenght(self) -> f64 {
        self.lenght_squared().sqrt()
    }

    pub const fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross(self, other: Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit(self) -> Self {
        self / self.lenght()
    }

    pub fn random_unit() -> Self {
        let range = Interval::new(-1.0, 1.0);
        let mut p = Self::random_range(range);
        let mut lensq = p.lenght_squared();

        while lensq <= 1e-160 || lensq > 1.0 {
            p = Self::random_range(range);
            lensq = p.lenght_squared();
        }

        p / lensq.sqrt()
    }

    pub fn random_in_disk() -> Self {
        let range = Interval::new(-1.0, 1.0);
        let mut p = Self::new(random_range(range), random_range(range), 0.0);
        let mut lensq = p.lenght_squared();

        while lensq > 1.0 {
            p = Self::random_range(range);
            lensq = p.lenght_squared();
        }

        p
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit = Self::random_unit();
        if on_unit.dot(normal) > 0.0 {
            on_unit
        } else {
            -on_unit
        }
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - (2.0 * self.dot(normal)) * normal
    }

    pub fn invert(self) -> Self {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = self.neg().dot(normal).min(1.0);
        let r_perpendicular = etai_over_etat * (self + cos_theta * normal);
        let r_parallel = -f64::sqrt(f64::abs(1.0 - r_perpendicular.lenght_squared())) * normal;
        r_perpendicular + r_parallel
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, v| acc + v)
    }
}
