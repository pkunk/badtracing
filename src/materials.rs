use crate::ray::Ray;
use crate::{random_unit_vec3, random_vec3_in_unit_sphere, Color, HitRecord};

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
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl MaterialProperties for Lambertian {
    fn scatter<R: Rng>(&self, rng: &mut R, r: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vec3(rng);

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
            reflected + self.fuzz * random_vec3_in_unit_sphere(rng),
        );
        Some((self.albedo, scattered))
    }
}
