use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::fmt::Display;

use badtracing::camera::Camera;
use badtracing::materials::{Dielectric, Lambertian, Metal};
use badtracing::objects::{Cube, Object, Sphere};
use badtracing::vec3::Vec3;
use badtracing::{random_f64, random_f64_mm, ray_color, write_color, Color, Point3};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::UNIX_EPOCH;

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = 800;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let aspect_ratio = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let progress_counter = AtomicI32::new(0);
    print_progress(IMAGE_HEIGHT);

    let render: Vec<Color> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let mut rng = SmallRng::seed_from_u64(j as u64);
            let mut scanline = Vec::with_capacity(IMAGE_WIDTH as usize);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::default();
                for _s in 0..SAMPLES_PER_PIXEL {
                    let u = (f64::from(i) + random_f64(&mut rng)) / f64::from(IMAGE_WIDTH);
                    let v = (f64::from(j) + random_f64(&mut rng)) / f64::from(IMAGE_HEIGHT);
                    let r = cam.get_ray(&mut rng, u, v);
                    pixel_color += ray_color(&mut rng, r, world.as_ref(), MAX_DEPTH);
                }
                scanline.push(pixel_color);
            }
            let finished = progress_counter.fetch_add(1, Ordering::AcqRel) + 1;
            print_progress(IMAGE_HEIGHT - finished);
            scanline
        })
        .collect();

    // Write
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    render
        .into_iter()
        .for_each(|pixel_color| write_color(pixel_color, SAMPLES_PER_PIXEL));

    eprintln!("\nDone.");
}

fn print_progress<D: Display>(remaining: D) {
    eprint!("\rScanlines remaining: {} ", remaining);
}

fn random_scene() -> Vec<Object> {
    let mut world = Vec::new();

    let ground_material = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    }
    .into();
    world.push(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material).into());

    let seed = UNIX_EPOCH.elapsed().unwrap().as_secs();
    eprintln!("World seed: {}", seed);
    let mut rng = SmallRng::seed_from_u64(seed);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(&mut rng),
                0.0,
                b as f64 + 0.9 * random_f64(&mut rng),
            );

            if (center - Point3::new(4.0, 0.0, 0.0)).length() > 0.9 {
                let material;
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo: Color = Color::random(&mut rng) * Color::random(&mut rng);
                    material = Lambertian { albedo }.into();
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo: Color = Color::random_mm(&mut rng, 0.5, 1.0);
                    let fuzz = random_f64_mm(&mut rng, 0.0, 0.5);
                    material = Metal { albedo, fuzz }.into();
                } else {
                    // glass
                    material = Dielectric { ir: 1.5 }.into();
                }
                let direction = random_f64(&mut rng);
                if direction > 0.2 {
                    world.push(
                        Sphere::new(center + Point3::new(0.0, 0.2, 0.0), 0.2, material).into(),
                    )
                } else {
                    world.push(
                        Cube::new(
                            center + Point3::new(0.0, 0.16, 0.0),
                            0.16,
                            Vec3::new(0.0, 1.0, 0.0),
                            Vec3::new(1.0 - direction, 0.0, 0.0),
                            material,
                        )
                        .into(),
                    )
                }
            }
        }
    }

    world.push(
        Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric { ir: 1.5 }.into(),
        )
        .into(),
    );

    world.push(
        Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian {
                albedo: Color::new(0.4, 0.2, 0.1),
            }
            .into(),
        )
        .into(),
    );

    world.push(
        Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }
            .into(),
        )
        .into(),
    );

    world
}
