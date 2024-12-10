use std::cmp::Ordering;

use crate::{interval::Interval, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    pub const fn all() -> [Axis; 3] {
        Self::ALL
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub const fn from_points(a: Vec3, b: Vec3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        Self::new(x, y, z)
    }

    pub const fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() && self.x.size() > self.z.size() {
            Axis::X
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub const fn merge(self, b: Self) -> Self {
        let x = self.x.merge(b.x);
        let y = self.y.merge(b.y);
        let z = self.z.merge(b.z);
        Self::new(x, y, z)
    }

    pub const fn x(&self) -> Interval {
        self.x
    }

    pub const fn y(&self) -> Interval {
        self.y
    }

    pub const fn z(&self) -> Interval {
        self.z
    }

    pub const fn get(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    #[inline(always)]
    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let origin = ray.origin();
        let dir = ray.direction();

        for axis_i in Axis::all() {
            let axis = self.get(axis_i);
            let adinv = 1.0 / dir.get(axis_i);

            let t0 = (axis.min() - origin.get(axis_i)) * adinv;
            let t1 = (axis.max() - origin.get(axis_i)) * adinv;

            let mut min = ray_t.min();
            let mut max = ray_t.max();

            if t0 < t1 {
                if t0 > ray_t.min() {
                    min = t0;
                }
                if t1 < ray_t.max() {
                    max = t1;
                }
            } else {
                if t1 > ray_t.min() {
                    min = t1;
                }
                if t0 < ray_t.max() {
                    max = t0;
                }
            }

            if max <= min {
                return false;
            }

            ray_t = Interval::new(min, max);
        }

        true
    }

    pub fn compare(&self, other: &Self, axis: Axis) -> Ordering {
        let axis_l = self.get(axis);
        let axis_r = other.get(axis);
        axis_l.min().total_cmp(&axis_r.min())
    }
}
