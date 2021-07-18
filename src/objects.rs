use crate::materials::Material;
use crate::ray::Ray;
use crate::{HitRecord, Point3};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Vec<Object> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter()
            .flat_map(|h| h.hit(r, t_min, t_max))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }
}

#[enum_dispatch(Hittable)]
#[derive(Copy, Clone, Debug)]
pub enum Object {
    Sphere,
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, r.dir, t, outward_normal, self.material))
    }
}
