pub mod ray;
pub mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    // Write the translated [0,255] value of each color component.
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    )
}

pub fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.orig - center;
    let a = r.dir.length_squared();
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
