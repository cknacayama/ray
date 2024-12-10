use crate::{
    aabb::Aabb,
    hit::{Hit, HitRecord},
    interval::Interval,
    material::Scatter,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere<T> {
    bbox:     Aabb,
    center:   Vec3,
    radius:   f64,
    material: T,
}

impl<T: Scatter> Sphere<T> {
    pub fn new(center: Vec3, radius: f64, material: T) -> Self {
        assert!(radius.is_sign_positive());
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(center - rvec, center + rvec);

        Self {
            center,
            radius: radius.max(0.0),
            material,
            bbox,
        }
    }

    pub const fn center(&self) -> Vec3 {
        self.center
    }

    pub const fn radius(&self) -> f64 {
        self.radius
    }

    pub const fn bbox(&self) -> Aabb {
        self.bbox
    }

    pub const fn material(&self) -> T
    where
        T: Copy,
    {
        self.material
    }
}

impl<T: Copy + Scatter> Hit for Sphere<T> {
    type Material = T;

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        let cur_center = self.center;
        let oc = cur_center - ray.origin();
        let a = ray.direction().lenght_squared();
        let h = ray.direction().dot(oc);
        let c = oc.lenght_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();

        let mut root = (h - dsqrt) / a;

        if !ray_t.surrounds(root) {
            root = (h + dsqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - cur_center) / self.radius;

        Some(HitRecord::new(point, normal, root, ray, self.material))
    }

    fn aabb(&self) -> Aabb {
        self.bbox
    }
}
