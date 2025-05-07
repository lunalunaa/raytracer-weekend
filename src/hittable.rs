use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::Material,
    ray::{FaceNormal, Interval, Ray},
    vector::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3,
    pub face_normal: FaceNormal,
    pub mat: Arc<dyn Material + Sync + Send>,
}

impl HitRecord {
    #[inline(always)]
    pub fn calc_face_normal(r: &Ray, outward_normal: &Vec3) -> FaceNormal {
        let front_face = r.dir.dot(outward_normal) < 0.;

        if front_face {
            FaceNormal::Front(*outward_normal)
        } else {
            FaceNormal::Back(-*outward_normal)
        }
    }

    #[inline(always)]
    pub fn normal(&self) -> &Vec3 {
        self.face_normal.normal()
    }

    #[inline(always)]
    pub fn new(
        t: f32,
        p: Point3,
        face_normal: FaceNormal,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            t,
            p,
            face_normal,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, int: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
    pub bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.bbox = Aabb::enclose(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    #[inline(always)]
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

    #[inline(always)]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
