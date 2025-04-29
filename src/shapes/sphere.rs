use crate::{
    ray::{HitRecord, Hittable, Interval, Ray},
    vector::Point3,
};

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Sphere {
            centre: centre,
            radius: radius.max(0.),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, intvl: &Interval) -> Option<HitRecord> {
        let oc = self.centre - r.origin;
        let a = r.dir.len_squared();
        let h = r.dir.dot(&oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discrim = h * h - a * c;
        if discrim < 0. {
            return None;
        }

        let sqrtd = discrim.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !intvl.surrounds(root) {
            root = (h + sqrtd) / a;
            if !intvl.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.centre) / self.radius;
        let face_normal = HitRecord::calc_face_normal(r, &outward_normal);

        Some(HitRecord::new(t, p, face_normal))
    }
}
