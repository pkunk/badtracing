use badtracing::camera::Camera;
use badtracing::objects::Sphere;
use badtracing::{ray_color, write_color, Color, Hittable, Point3};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = 255;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // Camera
    let cam = Camera::default();

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        let mut rng = SmallRng::seed_from_u64(j as u64);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
