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

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
