use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{aabb::Axis, interval::Interval, random_0_1, random_range};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<T = f64> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn x(self) -> T {
        self.x
    }

    pub const fn y(self) -> T {
        self.y
    }

    pub const fn z(self) -> T {
        self.z
    }

    pub const fn get(&self, axis: Axis) -> T {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

impl Vec3<f64> {
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

    pub const fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
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
        self / self.length()
    }

    pub fn random_unit() -> Self {
        let range = Interval::new(-1.0, 1.0);
        let mut p = Self::random_range(range);
        let mut lensq = p.length_squared();

        while lensq <= 1e-160 || lensq > 1.0 {
            p = Self::random_range(range);
            lensq = p.length_squared();
        }

        p / lensq.sqrt()
    }

    pub fn random_in_disk() -> Self {
        let range = Interval::new(-1.0, 1.0);
        let mut p = Self::new(random_range(range), random_range(range), 0.0);
        let mut lensq = p.length_squared();

        while lensq > 1.0 {
            p = Self::random_range(range);
            lensq = p.length_squared();
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
        self - normal * (2.0 * self.dot(normal))
    }

    pub fn invert(self) -> Self {
        Self::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }

    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = self.neg().dot(normal).min(1.0);
        let r_perpendicular = (self + normal * cos_theta) * etai_over_etat;
        let r_parallel = normal * -f64::sqrt(f64::abs(1.0 - r_perpendicular.length_squared()));
        r_perpendicular + r_parallel
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Vec3<T>
where
    f64: Div<T, Output = T> + Copy,
    Vec3<T>: Mul<T, Output = Self>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<T> Sum for Vec3<T>
where
    Vec3<T>: Add<Output = Self> + Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, v| acc + v)
    }
}
