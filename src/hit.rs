use std::{fmt::Debug, sync::Arc};

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::{Material, Scatter},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord<T> {
    point:      Vec3,
    normal:     Vec3,
    t:          f64,
    material:   T,
    front_face: bool,
}

impl<T: Scatter> HitRecord<T> {
    pub fn new(point: Vec3, normal: Vec3, t: f64, ray: &Ray, material: T) -> Self {
        let front_face = ray.direction().dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn scatter(&self, ray: &Ray) -> Option<(Vec3, Ray)> {
        self.material.scatter(ray, self)
    }

    pub fn emit(&self) -> Option<Vec3> {
        self.material.emit()
    }
}

pub trait Hit {
    type Material: Scatter;

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>>;
    fn aabb(&self) -> Aabb;
    fn count(&self) -> usize;
}

pub trait DynHit: Hit<Material = Material> + Send + Sync + Debug {}
impl<T: Hit<Material = Material> + Send + Sync + Debug> DynHit for T {
}

pub struct HitList {
    list: Vec<Arc<dyn DynHit>>,
    bbox: Aabb,
}

impl HitList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            bbox: Aabb::default(),
        }
    }

    pub fn push<T: DynHit + 'static>(&mut self, v: T) {
        self.bbox = self.bbox.merge(v.aabb());
        self.list.push(Arc::new(v));
    }

    pub fn list(&self) -> &[Arc<dyn DynHit>] {
        &self.list
    }

    pub fn list_mut(&mut self) -> &mut [Arc<dyn DynHit>] {
        &mut self.list
    }
}

impl Hit for HitList {
    type Material = Material;
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<Self::Material>> {
        let mut rec = None;
        let mut closest = ray_t.max();

        for obj in self.list.iter() {
            if let Some(hit) = obj.hit(ray, Interval::new(ray_t.min(), closest)) {
                closest = hit.t;
                rec = Some(hit);
            }
        }

        rec
    }

    fn aabb(&self) -> Aabb {
        self.bbox
    }

    fn count(&self) -> usize {
        self.list.len()
    }
}
