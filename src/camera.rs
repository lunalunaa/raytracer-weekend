use std::{fs::File, io::BufWriter};

use crate::{
    color::Color,
    material::Scatter,
    ray::{Hittable, Interval, Ray},
    vector::{Point3, Vec3},
};

use anyhow::Result;
use image::Rgb;

#[allow(unused)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    pub samples_per_pixel: usize,
    pixel_samples_scale: f64,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_bounce_depth: usize,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: usize,
        max_bounce_depth: usize,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let centre = Point3::zero();

        // calculate viewport edge vectors
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // calculate pixel deltas
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // calculate upper left pixel coordinate
        let viewport_upper_left =
            centre - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_bounce_depth,
        }
    }

    #[inline]
    fn render_pixel(&self, x: u32, y: u32, world: &(impl Hittable + Sync)) -> Rgb<u8> {
        let mut color = Color::zero();
        for _ in 0..self.samples_per_pixel {
            let r = self.get_ray(x, y);
            color += Self::ray_color(&r, world, self.max_bounce_depth);
        }

        image::Rgb((color * self.pixel_samples_scale).as_rgb().as_array())
    }

    pub fn render(&self, world: &(impl Hittable + Sync)) -> Result<()> {
        let bar = indicatif::ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        let img = image::ImageBuffer::from_par_fn(self.image_width, self.image_height, |x, y| {
            bar.inc(1);
            self.render_pixel(x, y, world)
        });

        let mut buf = BufWriter::new(File::create("image.png")?);
        img.write_to(&mut buf, image::ImageFormat::Png)?;
        Ok(())
    }

    // return a pair within [-0.5, 0.5], [-0.5, 0.5] range
    #[inline]
    fn sample_square() -> (f64, f64) {
        (
            rand::random_range(-0.5..=0.5),
            rand::random_range(-0.5..=0.5),
        )
    }

    // return the ray from the cam centre to the pixel coord (i, j)
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);
        let ray_dir = pixel_sample - self.centre;

        Ray::new(self.centre, ray_dir)
    }

    fn ray_color(r: &Ray, world: &impl Hittable, bounce_depth: usize) -> Color {
        if bounce_depth == 0 {
            return Color::zero();
        }

        if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            if let Scatter::Scattered(r, atten) = rec.mat.scatter(r, &rec) {
                return atten * Self::ray_color(&r, world, bounce_depth - 1);
            } else {
                return Color::zero();
            }
        }

        // otherwise a gradient background
        let unit_dir = r.dir.unit_vec();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
    }
}
