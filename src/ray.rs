use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin:    Vec3,
    direction: Vec3,
    time:      f64,
}

impl Ray {
    pub const fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction: dir,
            time,
        }
    }

    pub const fn origin(&self) -> Vec3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub const fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
