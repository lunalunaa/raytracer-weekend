use crate::vector::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

#[allow(unused)]
impl Ray {
    #[inline]
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
