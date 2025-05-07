use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    #[inline]
    pub const fn one() -> Self {
        Self::new(1., 1., 1.)
    }

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub const fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub const fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub const fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn unit_vec(&self) -> Vec3 {
        *self / self.len()
    }

    #[inline]
    pub fn random_cube() -> Vec3 {
        Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32())
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            fastrand_contrib::f32_range(min..=max),
            fastrand_contrib::f32_range(min..=max),
            fastrand_contrib::f32_range(min..=max),
        )
    }

    #[inline]
    pub fn random_unit_vec() -> Vec3 {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.len_squared();
            if 1e-160 < lensq && lensq <= 1. {
                return p / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vec();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    #[inline]
    pub const fn near_zero(&self) -> bool {
        let s = 1e-8;

        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - (2. * self.dot(n)) * *n
    }

    #[inline]
    // assume self is unit length
    pub fn refract(&self, n: &Vec3, eta_ratio: f32) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = eta_ratio * (*self + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs().sqrt()) * *n;

        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                fastrand_contrib::f32_range(-1.0..1.0),
                fastrand_contrib::f32_range(-1.0..1.0),
                0.,
            );
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// Hadamard product
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}
