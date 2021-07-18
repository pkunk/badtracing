use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;

use badtracing::camera::Camera;
use badtracing::materials::{Dielectric, Lambertian, Metal};
use badtracing::objects::{Object, Sphere};
use badtracing::vec3::Vec3;
use badtracing::{random_f64, ray_color, write_color, Color, Point3};

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = 255;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric { ir: 1.5 };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let world: Vec<Object> = vec![
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground.into(),
        }
        .into(),
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center.into(),
        }
        .into(),
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left.into(),
        }
        .into(),
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.4,
            material: material_left.into(),
        }
        .into(),
        Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right.into(),
        }
        .into(),
    ];

    // Camera
    let aspect_ratio = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let render: Vec<Color> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .flat_map(|j| {
            let mut rng = SmallRng::seed_from_u64(j as u64);
            let mut scanline = Vec::with_capacity(IMAGE_WIDTH as usize);
            for i in (0..IMAGE_WIDTH).rev() {
                let mut pixel_color = Color::default();
                for _s in 0..SAMPLES_PER_PIXEL {
                    let u = (f64::from(i) + random_f64(&mut rng)) / f64::from(IMAGE_WIDTH);
                    let v = (f64::from(j) + random_f64(&mut rng)) / f64::from(IMAGE_HEIGHT);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&mut rng, r, &world, MAX_DEPTH);
                }
                scanline.push(pixel_color);
            }
            scanline
        })
        .collect();
    render
        .into_iter()
        .rev()
        .for_each(|pixel_color| write_color(pixel_color, SAMPLES_PER_PIXEL));

    eprintln!("\nDone.");
}
