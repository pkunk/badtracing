pub mod camera;
pub mod objects;
pub mod ray;
pub mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, direction: Vec3, t: f64, outward_normal: Vec3) -> Self {
        let front_face = direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable + Sync + Send>> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter()
            .flat_map(|h| h.hit(r, t_min, t_max))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).clamp(0.0, 0.999) * 256.0;
    g = (g * scale).clamp(0.0, 0.999) * 256.0;
    b = (b * scale).clamp(0.0, 0.999) * 256.0;

    // Write the translated [0,255] value of each color component.
    println!("{} {} {}", r as i32, g as i32, b as i32)
}

pub fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(rec) => 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = r.dir.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
