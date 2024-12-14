use std::ops::Neg;

use crate::{hit::HitRecord, random_0_1, ray::Ray, vec3::Vec3};

pub trait Scatter: Sized {
    fn scatter<T: Scatter>(&self, ray: &Ray, hit: &HitRecord<T>) -> Option<(Vec3, Ray)>;
    fn emit(&self) -> Option<Vec3> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    albedo: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metal {
    albedo: Vec3,
    fuzz:   f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dielectric {
    refraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiffuseLight {
    color: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Metal {
    pub const fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Lambertian {
    pub const fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Dielectric {
    pub const fn new(refraction: f64) -> Self {
        Self { refraction }
    }

    pub fn reflectance(self, cos: f64) -> f64 {
        let r0 = (1.0 - self.refraction) / (1.0 + self.refraction);
        let r0 = r0 * r0;
        r0 * (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl DiffuseLight {
    pub const fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material {
    pub const fn lambertian(albedo: Vec3) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }

    pub const fn metal(albedo: Vec3, fuzz: f64) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub const fn dialectric(refraction: f64) -> Self {
        Material::Dielectric(Dielectric::new(refraction))
    }

    pub const fn diffuse_light(color: Vec3) -> Self {
        Material::DiffuseLight(DiffuseLight::new(color))
    }

    /// Returns `true` if the material is [`Dielectric`].
    ///
    /// [`Dielectric`]: Material::Dielectric
    #[must_use]
    pub const fn is_dielectric(&self) -> bool {
        matches!(self, Self::Dielectric(..))
    }
}

impl Scatter for Lambertian {
    fn scatter<T: Scatter>(&self, ray: &Ray, hit: &HitRecord<T>) -> Option<(Vec3, Ray)> {
        let mut dir = hit.normal() + Vec3::random_unit();
        if dir.near_zero() {
            dir = hit.normal();
        }
        let scattered = Ray::new(hit.point(), dir, ray.time());
        Some((self.albedo, scattered))
    }
}

impl Scatter for Metal {
    fn scatter<T: Scatter>(&self, ray: &Ray, hit: &HitRecord<T>) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().reflect(hit.normal());
        let reflected = reflected.unit() + (Vec3::random_unit() * self.fuzz);
        let scattered = Ray::new(hit.point(), reflected, ray.time());
        if scattered.direction().dot(hit.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Scatter for Dielectric {
    fn scatter<T: Scatter>(&self, ray: &Ray, hit: &HitRecord<T>) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if hit.front_face() {
            1.0 / self.refraction
        } else {
            self.refraction
        };
        let unit_dir = ray.direction().unit();
        let cos_theta = unit_dir.neg().dot(hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 || self.reflectance(cos_theta) > random_0_1() {
            unit_dir.reflect(hit.normal())
        } else {
            unit_dir.refract(hit.normal(), ri)
        };

        let scattered = Ray::new(hit.point(), direction, ray.time());

        Some((attenuation, scattered))
    }
}

impl Scatter for DiffuseLight {
    fn scatter<T: Scatter>(&self, _: &Ray, _: &HitRecord<T>) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self) -> Option<Vec3> {
        Some(self.color)
    }
}

impl Scatter for Material {
    fn scatter<T: Scatter>(&self, ray: &Ray, hit: &HitRecord<T>) -> Option<(Vec3, Ray)> {
        match self {
            Material::Metal(metal) => metal.scatter(ray, hit),
            Material::Lambertian(lambertian) => lambertian.scatter(ray, hit),
            Material::Dielectric(dielectric) => dielectric.scatter(ray, hit),
            Material::DiffuseLight(light) => light.scatter(ray, hit),
        }
    }

    fn emit(&self) -> Option<Vec3> {
        match self {
            Material::Metal(metal) => metal.emit(),
            Material::Lambertian(lambertian) => lambertian.emit(),
            Material::Dielectric(dielectric) => dielectric.emit(),
            Material::DiffuseLight(diffuse_light) => diffuse_light.emit(),
        }
    }
}
