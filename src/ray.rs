use crate::vector::{Point3, Vec3};

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
    pub fn at(&self, t: f32) -> Point3 {
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
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

#[allow(unused)]
impl Interval {
    #[inline]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[inline]
    pub const fn size(&self) -> f32 {
        self.max - self.min
    }

    #[inline]
    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline]
    pub const fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[inline]
    pub const fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    #[inline]
    pub const fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.;
        Self::new(self.min - padding, self.max + padding)
    }

    #[inline]
    pub const fn enclose(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };

        let max = if a.max >= b.max { a.max } else { b.max };

        Self::new(min, max)
    }

    pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);
}
