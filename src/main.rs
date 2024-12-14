use std::fs::OpenOptions;

use ray::{
    bvh::Bvh,
    camera::Camera,
    geo::{Quad, Sphere, Triangle},
    hit::{Hit, HitList},
    interval::Interval,
    material::Material,
    random_0_1, random_range,
    vec3::Vec3,
};

fn spheres() -> Bvh {
    let mut world = HitList::new();

    let material_ground = Material::metal(Vec3::new(0.7, 0.7, 0.7), 0.01);
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);

    world.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let mat = random_0_1();
            let center = Vec3::new(
                a as f64 + 0.8 * random_0_1(),
                0.2,
                b as f64 + 0.8 * random_0_1(),
            );

            if (center - Vec3::new(4.0, 1.0, 0.0)).length() > 1.2
                && (center - Vec3::new(0.0, 1.0, 0.0)).length() > 1.2
                && (center - Vec3::new(-4.0, 1.0, 0.0)).length() > 1.2
            {
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

    Bvh::from_list(world.list_mut())
}

fn triangles() -> Bvh {
    let mut world = HitList::new();

    let material_ground = Material::lambertian(Vec3::new(0.5, 0.5, 0.5));
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    world.push(ground);

    let metal = Vec3::new(0.9, 0.9, 0.9);
    let material = Material::metal(metal, 0.0);

    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(-8.0, 0.0, 0.0);
    let c = Vec3::new(-8.0, 10.0, 0.0);

    let triangle = Triangle::new(a, b, c, material);
    world.push(triangle);

    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 10.0, 0.0);
    let c = Vec3::new(-8.0, 10.0, 0.0);

    let triangle = Triangle::new(a, b, c, material);
    world.push(triangle);

    let material = Material::diffuse_light(Vec3::new(1.0, 1.0, 1.0));
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 2.5), 1.0, material);
    world.push(sphere);

    Bvh::from_list(world.list_mut())
}

fn quads() -> Bvh {
    let mut world = HitList::new();

    let left_red = Material::lambertian(Vec3::new(1.0, 0.2, 0.2));
    let back_green = Material::lambertian(Vec3::new(0.2, 1.0, 0.2));
    let right_blue = Material::lambertian(Vec3::new(0.2, 0.2, 1.0));
    let upper_orange = Material::lambertian(Vec3::new(1.0, 0.5, 0.0));
    let lower_teal = Material::lambertian(Vec3::new(0.2, 0.8, 0.8));

    let left = Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    );
    let back = Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    );
    let right = Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    );
    let upper = Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    );
    let lower = Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    );

    world.push(left);
    world.push(right);
    world.push(back);
    world.push(upper);
    world.push(lower);

    Bvh::from_list(world.list_mut())
}

fn main() {
    let img_width = 2560u32;
    let aspect_ratio = 16.0 / 9.0;
    let sample_count = 700;
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

    let world = spheres();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("img/image5.ppm")
        .unwrap();

    camera.render(&world, sample_count, max_depth, file);
}
