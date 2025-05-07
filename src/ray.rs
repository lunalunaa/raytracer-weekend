use std::{cmp::Ordering, sync::Arc};

use rand::random_range;

use crate::{
    aabb::Aabb,
    material::Material,
    vector::{Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

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
    Front(Vec3), // ray hits surface "from the inside"
    Back(Vec3),  // ray hits surface "from the outside"
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

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub face_normal: FaceNormal,
    pub mat: Arc<dyn Material + Sync + Send>,
}

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

    pub fn new(
        t: f64,
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct BVHNode {
    pub left: Arc<dyn Hittable + Sync + Send>,
    pub right: Arc<dyn Hittable + Sync + Send>,
    pub bbox: Aabb,
}

impl BVHNode {
    pub fn from_hittable_list(hittable_list: HittableList) -> Self {
        let mut hittable_list = hittable_list;
        Self::from_object_slice(hittable_list.objects.as_mut_slice())
    }

    pub fn from_object_slice(objects: &mut [Arc<dyn Hittable + Sync + Send>]) -> Self {
        let axis = random_range(0..3);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = objects.len();
        let left;
        let right;

        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            left = objects[0].clone();
            right = objects[1].clone();
        } else {
            objects.sort_unstable_by(comparator);

            let mid = object_span / 2;
            left = Arc::new(Self::from_object_slice(&mut objects[..mid]));
            right = Arc::new(Self::from_object_slice(&mut objects[mid..]))
        }

        let bbox = Aabb::enclose(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }

    #[inline]
    fn box_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
        axis_idx: u8,
    ) -> Ordering {
        let a_axis_int = a.bounding_box().axis_interval(axis_idx);
        let b_axis_int = b.bounding_box().axis_interval(axis_idx);
        a_axis_int.min.total_cmp(&b_axis_int.min)
    }

    #[inline]
    fn box_x_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    #[inline]
    fn box_y_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    #[inline]
    fn box_z_compare(
        a: &Arc<dyn Hittable + Sync + Send>,
        b: &Arc<dyn Hittable + Sync + Send>,
    ) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, int: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, int) {
            return None;
        }

        let hit_left = self.left.hit(r, int);
        let int_right = if let Some(rec) = hit_left.as_ref() {
            &Interval::new(int.min, rec.t)
        } else {
            int
        };

        let hit_right = self.right.hit(r, int_right);

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

#[derive(Clone)]
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

    #[inline]
    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self::new(self.min - padding, self.max + padding)
    }

    #[inline]
    pub const fn enclose(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };

        let max = if a.max >= b.max { a.max } else { b.max };

        Self::new(min, max)
    }

    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}
