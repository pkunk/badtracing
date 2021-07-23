use rand::Rng;

use objects::Hittable;

use crate::materials::{Material, MaterialProperties};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod camera;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod vec3;

pub type Point3 = Vec3;
pub type UnitVec3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: UnitVec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, direction: Vec3, t: f64, outward_normal: UnitVec3, m: Material) -> Self {
        let front_face = direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            material: m,
            t,
            front_face,
        }
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    let ri = (r.clamp(0.0, 0.999) * 256.0) as i32;
    let gi = (g.clamp(0.0, 0.999) * 256.0) as i32;
    let bi = (b.clamp(0.0, 0.999) * 256.0) as i32;

    // Write the translated [0,255] value of each color component.
    println!("{} {} {}", ri, gi, bi)
}

pub fn ray_color<R: Rng, H: Hittable>(rng: &mut R, r: Ray, world: H, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::default();
    }
    match world.hit(r, 0.001, f64::INFINITY) {
        Some(rec) => match rec.material.scatter(rng, r, rec) {
            Some((attenuation, scattered)) => {
                attenuation * ray_color(rng, scattered, world, depth - 1)
            }
            None => Color::default(),
        },
        None => {
            let unit_direction = r.dir.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

pub fn random_f64<R: Rng>(rng: &mut R) -> f64 {
    rng.gen()
}

pub fn random_f64_mm<R: Rng>(rng: &mut R, min: f64, max: f64) -> f64 {
    min + (max - min) * rng.gen::<f64>()
}

pub fn random_vec3<R: Rng>(rng: &mut R) -> Vec3 {
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn random_vec3_mm<R: Rng>(rng: &mut R, min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_f64_mm(rng, min, max),
        random_f64_mm(rng, min, max),
        random_f64_mm(rng, min, max),
    )
}

pub fn random_vec3_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = random_vec3_mm(rng, -1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vec3<R: Rng>(rng: &mut R) -> Vec3 {
    random_vec3_in_unit_sphere(rng).unit_vector()
}

pub fn random_vec3_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f64_mm(rng, -1.0, 1.0),
            random_f64_mm(rng, -1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
