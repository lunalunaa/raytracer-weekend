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
    Unscattered,
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
}

impl Metal {
    #[inline]
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scatter {
        let reflected = r_in.dir.reflect(rec.normal());
        let r = Ray::new(rec.p, reflected);
        Scatter::Scattered(r, self.albedo)
    }
}
