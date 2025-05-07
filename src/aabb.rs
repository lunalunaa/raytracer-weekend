use crate::{
    ray::{Interval, Ray},
    vector::Point3,
};

pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    #[inline]
    pub const fn empty() -> Self {
        Aabb {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }

    #[inline]
    pub const fn new(a: &Point3, b: &Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self { x, y, z }
    }

    #[inline]
    pub const fn axis_interval(&self, n: u8) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    #[inline]
    pub const fn enclose(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            x: Interval::enclose(&box0.x, &box1.x),
            y: Interval::enclose(&box0.y, &box1.y),
            z: Interval::enclose(&box0.z, &box1.z),
        }
    }

    pub fn hit(&self, r: &Ray, int: &Interval) -> bool {
        let Ray { dir, origin } = r;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ray_dir_axis = match axis {
                0 => dir.x,
                1 => dir.y,
                _ => dir.z,
            };
            let ray_orig_axis = match axis {
                0 => origin.x,
                1 => origin.y,
                _ => origin.z,
            };
            let adinv = 1.0 / ray_dir_axis;
            let t0 = (ax.min - ray_orig_axis) * adinv;
            let t1 = (ax.max - ray_orig_axis) * adinv;

            let mut int = int.clone();
            if t0 < t1 {
                if t0 > int.min {
                    int.min = t0;
                }
                if t1 < int.max {
                    int.max = t1;
                }
            } else {
                if t1 > int.min {
                    int.min = t1;
                }
                if t0 < int.max {
                    int.max = t0;
                }
            }

            if int.max <= int.min {
                return false;
            }
        }

        return true;
    }
}
