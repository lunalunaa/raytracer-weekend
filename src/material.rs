use crate::{
    color::Color,
    ray::{HitRecord, Ray},
    vector::Vec3,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scatter;
}

#[allow(unused)]
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
    pub fuzz: f64,
}

impl Metal {
    #[inline]
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
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
