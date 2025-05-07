use crate::{color::Color, hittable::HitRecord, ray::Ray, vector::Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scatter;
}

pub enum Scatter {
    Scattered(Ray, Color), // scattered ray and attenuation
    Absorbed,
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    #[inline]
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Scatter {
        let mut scatter_dir = *rec.normal() + Vec3::random_unit_vec();

        if scatter_dir.near_zero() {
            scatter_dir = *rec.normal();
        }

        let r = Ray::new(rec.p, scatter_dir);

        Scatter::Scattered(r, self.albedo)
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    #[inline]
    pub const fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scatter {
        // fuzz factor adds randomness to the scattering
        let reflected =
            r_in.dir.reflect(rec.normal()).unit_vec() + (self.fuzz * Vec3::random_unit_vec());
        let r = Ray::new(rec.p, reflected);

        if r.dir.dot(rec.normal()) > 0. {
            Scatter::Scattered(r, self.albedo)
        } else {
            Scatter::Absorbed
        }
    }
}

// refractive index in vacuum or air
// or the ratio of the refractive index over the refractive index of the enclosing media
pub struct Dielectric {
    pub refract_idx: f32,
}

impl Dielectric {
    pub fn new(refract_idx: f32) -> Self {
        Self { refract_idx }
    }

    fn reflectance(cosine: f32, refract_idx: f32) -> f32 {
        let mut r_0 = (1. - refract_idx) / (1. + refract_idx);
        r_0 *= r_0;
        r_0 + (1. - r_0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scatter {
        let atten = Color::one();

        // if it hits the inner surface then we need to invert the index,
        // otherwise we keep it the same
        let ri = if rec.face_normal.is_front() {
            1.0 / self.refract_idx
        } else {
            self.refract_idx
        };

        let r_in_unit_dir = r_in.dir.unit_vec();

        let cos_theta = (-r_in_unit_dir).dot(rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let dir = if cannot_refract
            || Self::reflectance(cos_theta, ri) > fastrand_contrib::f32_range(0.0..1.0)
        {
            r_in_unit_dir.reflect(rec.normal())
        } else {
            r_in_unit_dir.refract(rec.normal(), ri)
        };

        let r = Ray::new(rec.p, dir);
        Scatter::Scattered(r, atten)
    }
}
