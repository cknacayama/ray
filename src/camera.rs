use std::{
    fmt::Debug,
    io::{BufWriter, Write},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{hit::Hit, interval::Interval, random_0_1, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    aspect_ratio:  f64,
    img_width:     u32,
    img_height:    u32,
    pixel00_loc:   Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocous_angle: f64,
    disk_u:         Vec3,
    disk_v:         Vec3,

    center:     Vec3,
    w:          Vec3,
    u:          Vec3,
    v:          Vec3,
    background: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        img_width: u32,
        fov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocous_angle: f64,
        focus_dist: f64,
        background: Vec3,
    ) -> Self {
        let img_height = (img_width as f64 / aspect_ratio) as u32;

        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        let center = lookfrom;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / (img_width as f64);
        let pixel_delta_v = viewport_v / (img_height as f64);

        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let disk_radius = focus_dist * (defocous_angle / 2.0).to_radians().tan();
        let disk_u = u * disk_radius;
        let disk_v = v * disk_radius;

        Self {
            aspect_ratio,
            img_width,
            img_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocous_angle,
            disk_u,
            disk_v,
            w,
            u,
            v,
            background,
        }
    }

    #[inline(always)]
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let i = i as f64;
        let j = j as f64;

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));

        let origin = if self.defocous_angle <= 0.0 {
            self.center
        } else {
            self.disk_sample()
        };
        let dir = pixel_sample - origin;
        let time = random_0_1();

        Ray::new(origin, dir, time)
    }

    fn disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_disk();
        self.center + (self.disk_u * p.x()) + (self.disk_v * p.y())
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_0_1() - 0.5, random_0_1() - 0.5, 0.0)
    }

    fn sample_par<T: Hit + ?Sized + Sync>(
        &self,
        i: u32,
        j: u32,
        sample_count: u32,
        world: &T,
        max_depth: u32,
    ) -> Vec3
    where
        T::Material: Copy,
    {
        (0..sample_count)
            .into_par_iter()
            .map(|_| {
                let ray = self.get_ray(i, j);
                Self::ray_color(&ray, world, max_depth, self.background)
            })
            .sum()
    }

    fn sample_seq<T: Hit + ?Sized>(
        &self,
        i: u32,
        j: u32,
        sample_count: u32,
        world: &T,
        max_depth: u32,
    ) -> Vec3
    where
        T::Material: Copy,
    {
        (0..sample_count)
            .into_iter()
            .map(|_| {
                let ray = self.get_ray(i, j);
                Self::ray_color(&ray, world, max_depth, self.background)
            })
            .sum()
    }

    pub fn render<T: Hit + ?Sized + Sync, W: Write>(
        &self,
        world: &T,
        sample_count: u32,
        max_depth: u32,
        writer: W,
    ) where
        T::Material: Copy,
    {
        let mut w = BufWriter::new(writer);

        let count = world.count() as f64;
        let count_log2 = (count).log2() as u32;

        writeln!(w, "P3\n{} {}\n255", self.img_width, self.img_height).unwrap();

        let sample_scale = 1.0 / sample_count as f64;
        for j in 0..self.img_height {
            eprintln!("\nScanlines remaining {}", self.img_height - j);
            for i in 0..self.img_width {
                let color = if (sample_count * count_log2) < 1000 {
                    self.sample_seq(i, j, sample_count, world, max_depth)
                } else {
                    self.sample_par(i, j, sample_count, world, max_depth)
                };
                let color = color * sample_scale;
                let (r, g, b) = color.to_color();
                writeln!(w, "{} {} {}", r, g, b).unwrap();
            }
        }
        w.flush().unwrap();

        eprintln!("\nDone.\n");
    }

    #[inline(always)]
    fn ray_color<T: Hit + ?Sized>(ray: &Ray, world: &T, depth: u32, background: Vec3) -> Vec3
    where
        T::Material: Copy,
    {
        if depth == 0 {
            return background;
        }
        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit) => match hit.scatter(ray) {
                Some((attenuation, scattered)) => {
                    Self::ray_color(&scattered, world, depth - 1, background).scale(attenuation)
                }
                None => hit.emit().unwrap_or(background),
            },
            None => background,
        }
    }
}
