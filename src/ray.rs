use crate::vec3::Vec3;
use crate::Point3;

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
