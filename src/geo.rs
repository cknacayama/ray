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
    center: Vec3,
    radius: f64,

    bbox:     Aabb,
    material: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle<T> {
    a: Vec3,
    b: Vec3,
    c: Vec3,

    normal:   Vec3,
    bbox:     Aabb,
    material: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad<T> {
    origin: Vec3,
    u:      Vec3,
    v:      Vec3,
    w:      Vec3,

    normal:   Vec3,
    bbox:     Aabb,
    material: T,
}

impl<T> Quad<T> {
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, material: T) -> Self {
        let bbox_d1 = Aabb::from_points(origin, origin + u + v);
        let bbox_d2 = Aabb::from_points(origin + u, origin + u);
        let bbox = bbox_d1.merge(bbox_d2);

        let n = u.cross(v);
        let normal = n.unit();
        let w = n / n.dot(n);

        Self {
            origin,
            u,
            v,
            w,
            bbox,
            normal,
            material,
        }
    }
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

impl<T> Triangle<T> {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: T) -> Self {
        let x = Interval::new(a.x().min(b.x()).min(c.x()), a.x().max(b.x()).max(c.x()));
        let y = Interval::new(a.y().min(b.y()).min(c.y()), a.y().max(b.y()).max(c.y()));
        let z = Interval::new(a.z().min(b.z()).min(c.z()), a.z().max(b.z()).max(c.z()));

        let bbox = Aabb::new(x, y, z);

        let e1 = b - a;
        let e2 = c - a;
        let normal = e1.cross(e2).unit();

        Self {
            material,
            a,
            b,
            c,
            normal,
            bbox,
        }
    }
}

impl<T: Copy + Scatter> Hit for Quad<T> {
    type Material = T;

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        let denom = self.normal.dot(ray.direction());

        if denom > -f64::EPSILON && denom < f64::EPSILON {
            return None;
        }

        let d = self.normal.dot(self.origin);
        let t = (d - self.normal.dot(ray.origin())) / denom;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection_point = ray.at(t);
        let planar_hit_point = intersection_point - self.origin;

        let alpha = self.w.dot(planar_hit_point.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_point));

        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection_point,
            self.normal,
            t,
            ray,
            self.material,
        ))
    }

    fn aabb(&self) -> Aabb {
        self.bbox
    }

    fn count(&self) -> usize {
        1
    }
}

impl<T: Copy + Scatter> Hit for Sphere<T> {
    type Material = T;

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        let cur_center = self.center;
        let oc = cur_center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
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

    fn count(&self) -> usize {
        1
    }
}

impl<T: Copy + Scatter> Hit for Triangle<T> {
    type Material = T;

    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        let origin = ray.origin();
        let direction = ray.direction();

        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this self.
        }

        let inv_det = 1.0 / det;
        let s = origin - self.a;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inv_det * e2.dot(s_cross_e1);

        if !ray_t.surrounds(t) {
            return None;
        }

        let intersection_point = origin + direction * t;

        Some(HitRecord::new(
            intersection_point,
            self.normal,
            t,
            ray,
            self.material,
        ))
    }

    fn aabb(&self) -> Aabb {
        self.bbox
    }

    fn count(&self) -> usize {
        1
    }
}
