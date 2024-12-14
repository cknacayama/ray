#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Interval<T = f64> {
    min: T,
    max: T,
}

impl<T: Copy> Interval<T> {
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub const fn min(&self) -> T {
        self.min
    }

    pub const fn max(&self) -> T {
        self.max
    }

    pub const fn min_mut(&mut self) -> &mut T {
        &mut self.min
    }

    pub const fn max_mut(&mut self) -> &mut T {
        &mut self.max
    }
}

impl Interval<f64> {
    pub const fn inf() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    pub const fn empty() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub const fn merge(&self, other: Self) -> Self {
        let min = self.min().min(other.min());
        let max = self.max().max(other.max());
        Self::new(min, max)
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
