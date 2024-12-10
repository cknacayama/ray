use std::fs::OpenOptions;

use ray::{
    bvh::Bvh, camera::Camera, geo::Sphere, hit::HitList, interval::Interval, material::Material,
    random_0_1, random_range, vec3::Vec3,
};

fn main() {
    let mut world = HitList::new();

    let material_ground = Material::lambertian(Vec3::new(0.5, 0.5, 0.5));
    let material_sun = Material::diffuse_light(Vec3::new(1.0, 1.0, 1.0));
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    let sun = Sphere::new(Vec3::new(0.0, 100.0, 0.0), 20.0, material_sun);

    world.push(ground);
    // world.push(sun);

    for a in -11..11 {
        for b in -11..11 {
            let mat = random_0_1();
            let center = Vec3::new(
                a as f64 + 0.9 * random_0_1(),
                0.2,
                b as f64 + 0.9 * random_0_1(),
            );

            if (center - Vec3::new(4.0, 0.0, 0.0)).lenght() > 1.2 {
                let material = if mat < 0.4 {
                    let albedo = Vec3::random().scale(Vec3::random());
                    Material::lambertian(albedo)
                } else if mat < 0.7 {
                    let color_interval = Interval::new(0.6, 1.0);
                    let fuzz_interval = Interval::new(0.0, 0.5);
                    let albedo = Vec3::random_range(color_interval);
                    let fuzz = random_range(fuzz_interval);
                    Material::metal(albedo, fuzz)
                } else if mat < 0.9 {
                    Material::dialectric(1.5)
                } else {
                    let interval = Interval::new(0.2, 1.0);
                    let color = Vec3::random_range(interval);
                    Material::diffuse_light(color)
                };
                let sphere = Sphere::new(center, 0.2, material);
                world.push(sphere);
                if material.is_dielectric() {
                    let material = Material::dialectric(1.0 / 1.5);
                    let sphere = Sphere::new(center, 0.2 * 0.6, material);
                    world.push(sphere);
                }
            }
        }
    }

    let material1 = Material::dialectric(1.5);
    let material1_0 = Material::dialectric(1.0 / 1.5);
    let material3 = Material::diffuse_light(Vec3::new(1.0, 1.0, 1.0));
    let material2 = Material::metal(Vec3::new(0.8, 0.8, 0.8), 0.0);

    let sphere1 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material1);
    let sphere1_0 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 0.6, material1_0);
    let sphere2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2);
    let sphere3 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material3);

    world.push(sphere1);
    world.push(sphere1_0);
    world.push(sphere2);
    world.push(sphere3);

    let img_width = 1920u32;
    let aspect_ratio = 16.0 / 9.0;
    let sample_count = 500;
    let max_depth = 50;
    let fov = 20.0;
    let lookfrom = Vec3::new(10.0, 20.0, 20.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 30.0;
    let background = Vec3::default();

    let camera = Camera::new(
        aspect_ratio,
        img_width,
        fov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
        background,
    );

    let world = Bvh::from_list(world.list_mut());

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("img/image3.ppm")
        .unwrap();

    camera.render(&world, sample_count, max_depth, file);
}
