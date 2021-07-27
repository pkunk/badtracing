use crate::materials::Material;
use crate::ray::Ray;
use crate::{HitRecord, Point3, UnitVec3};

use crate::vec3::Vec3;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for &[Object] {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut current_t = t_max;
        let mut result = None;
        for h in self.iter() {
            if let Some(hit) = h.hit(r, t_min, current_t) {
                if hit.t < current_t {
                    current_t = hit.t;
                    result = Some(hit);
                }
            }
        }
        result
    }
}

#[enum_dispatch(Hittable)]
#[derive(Copy, Clone, Debug)]
pub enum Object {
    Sphere,
    Square,
    Cube,
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
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

#[derive(Debug, Copy, Clone)]
pub struct Square {
    center: Point3,
    radius: f64,
    normal: UnitVec3,
    orientation: UnitVec3,
    material: Material,
}

impl Hittable for Square {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let d = r.dir.dot(self.normal);
        if d.abs() < 1e-16 {
            return None;
        }
        let a = -oc.dot(self.normal);

        let t = a / d;
        if t < t_min || t > t_max {
            return None;
        }
        let p = r.at(t);
        let op = p - self.center;
        let outward_normal = self.normal;
        if op.dot(self.orientation).abs() > self.radius {
            return None;
        }
        if op.dot(self.orientation.cross(self.normal)).abs() > self.radius {
            return None;
        }

        Some(HitRecord::new(p, r.dir, t, outward_normal, self.material))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cube {
    center: Point3,
    radius: f64,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    material: Material,
}

impl Cube {
    pub fn new(center: Point3, radius: f64, axis0: Vec3, axis1: Vec3, material: Material) -> Self {
        assert!(axis0.cross(axis1).length_squared() > 0.0);
        let a = axis0.unit_vector();
        let b = (axis1 - a * axis1.dot(a)).unit_vector();
        let c = a.cross(b);

        Self {
            center,
            radius,
            a,
            b,
            c,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if (r.orig - self.center).length() > r.dir.length() * t_max + 2.0 * self.radius {
            return None;
        }
        [
            Square {
                center: self.center + self.radius * self.a,
                radius: self.radius,
                normal: self.a,
                orientation: self.b,
                material: self.material,
            }
            .into(),
            Square {
                center: self.center - self.radius * self.a,
                radius: self.radius,
                normal: self.a,
                orientation: self.b,
                material: self.material,
            }
            .into(),
            Square {
                center: self.center + self.radius * self.b,
                radius: self.radius,
                normal: self.b,
                orientation: self.c,
                material: self.material,
            }
            .into(),
            Square {
                center: self.center - self.radius * self.b,
                radius: self.radius,
                normal: self.b,
                orientation: self.c,
                material: self.material,
            }
            .into(),
            Square {
                center: self.center + self.radius * self.c,
                radius: self.radius,
                normal: self.c,
                orientation: self.a,
                material: self.material,
            }
            .into(),
            Square {
                center: self.center - self.radius * self.c,
                radius: self.radius,
                normal: self.c,
                orientation: self.a,
                material: self.material,
            }
            .into(),
        ]
        .as_ref()
        .hit(r, t_min, t_max)
    }
}
