use crate::ray::Ray;
use crate::{random_f64, Color, HitRecord};

use crate::vec3::Vec3;
use enum_dispatch::enum_dispatch;
use rand::Rng;

#[enum_dispatch]
pub trait MaterialProperties {
    fn scatter<R: Rng>(&self, rng: &mut R, r: Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}

#[enum_dispatch(MaterialProperties)]
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl MaterialProperties for Lambertian {
    fn scatter<R: Rng>(&self, rng: &mut R, _r: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl MaterialProperties for Metal {
    fn scatter<R: Rng>(&self, rng: &mut R, r: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.dir.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
        );
        Some((self.albedo, scattered))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    fn reflectance(self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl MaterialProperties for Dielectric {
    fn scatter<R: Rng>(&self, rng: &mut R, r: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r.dir.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64(rng) {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
