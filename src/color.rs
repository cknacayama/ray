use crate::{interval::Interval, vec3::Vec3};

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl Vec3 {
    pub fn to_color(self) -> (u8, u8, u8) {
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        const INTENSITY: Interval = Interval::new(0.000, 0.999);

        let r = (256.0 * INTENSITY.clamp(r)) as u8;
        let g = (256.0 * INTENSITY.clamp(g)) as u8;
        let b = (256.0 * INTENSITY.clamp(b)) as u8;

        (r, g, b)
    }
}
