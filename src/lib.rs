pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod geo;
pub mod hit;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

use interval::Interval;
use rand::{Rng, distributions::Uniform};

pub fn random_0_1() -> f64 {
    let distr = Uniform::new(0.0, 1.0);
    rand::thread_rng().sample(distr)
}

pub fn random_range(interval: Interval) -> f64 {
    let distr = Uniform::new(interval.min(), interval.max());
    rand::thread_rng().sample(distr)
}
