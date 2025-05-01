use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

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

// normals always point against the ray
// we keep track of which side of the surface the ray is coming from
#[derive(Clone)]
pub enum FaceNormal {
    Front(Vec3),
    Back(Vec3),
}

#[allow(unused)]
impl FaceNormal {
    pub fn is_front(&self) -> bool {
        match self {
            FaceNormal::Front(_) => true,
            FaceNormal::Back(_) => false,
        }
    }

    pub fn is_back(&self) -> bool {
        !self.is_front()
    }

    pub fn normal(&self) -> &Vec3 {
        match self {
            FaceNormal::Front(normal) => normal,
            FaceNormal::Back(normal) => normal,
        }
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub face_normal: FaceNormal,
}

#[allow(unused)]
impl HitRecord {
    pub fn calc_face_normal(r: &Ray, outward_normal: &Vec3) -> FaceNormal {
        let front_face = r.dir.dot(outward_normal) < 0.;

        if front_face {
            FaceNormal::Front(*outward_normal)
        } else {
            FaceNormal::Back(-*outward_normal)
        }
    }

    pub fn normal(&self) -> &Vec3 {
        self.face_normal.normal()
    }

    pub fn new(t: f64, p: Point3, face_normal: FaceNormal) -> Self {
        Self { t, p, face_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, int: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, intvl: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = intvl.max;
        let mut rec = None;

        for object in &self.objects {
            let temp_rec = object.hit(r, &Interval::new(intvl.min, closest_so_far));

            if let Some(temp_rec1) = temp_rec {
                closest_so_far = temp_rec1.t;
                rec = Some(temp_rec1)
            }
        }

        rec
    }
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(unused)]
impl Interval {
    #[inline]
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[inline]
    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    #[inline]
    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline]
    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    #[inline]
    pub const fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

#[allow(unused)]
pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
#[allow(unused)]
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
