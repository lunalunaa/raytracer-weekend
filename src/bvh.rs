use std::{cmp::Ordering, sync::Arc};

use crate::hittable::HitRecord;
use crate::{
    aabb::Aabb,
    hittable::{Hittable, HittableList},
    ray::{Interval, Ray},
};

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
        let mut bbox = Aabb::empty();
        for obj in objects.iter() {
            bbox = Aabb::enclose(&bbox, obj.bounding_box())
        }

        let axis = bbox.longest_axis();

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
