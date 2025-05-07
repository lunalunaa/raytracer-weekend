use std::{f64, fs::File, io::BufWriter, time::Instant};

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
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: u32,         // Rendered image width in pixel count
    image_height: u32,            // Rendered image height
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pixel_samples_scale: f64,     // Color scale factor for a sum of pixel samples
    centre: Point3,               // Camera center
    pixel00_loc: Point3,          // Location of pixel 0, 0
    pixel_delta_u: Vec3,          // Offset to pixel to the right
    pixel_delta_v: Vec3,          // Offset to pixel below
    max_bounce_depth: usize,      // Maximal number of bounces for a ray
    pub vfov: f64,                // Vertial field of view
    pub lookfrom: Point3,         // Point camera is looking from
    pub lookat: Point3,           // Point camera is looking at
    pub vup: Vec3,                // Camera-relative "up" direction
    pub defocus_angle: f64,       // Variation angle of rays through each pixel
    pub focus_dist: f64,          // Distance from camera lookfrom point to plane of perfect focus
    // camera frame basis vecs
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: usize,
        max_bounce_depth: usize,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let centre = lookfrom;

        // determine viewport dimensions

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        // calculate viewport edge vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-v);

        // calculate pixel deltas
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // calculate upper left pixel coordinate
        let viewport_upper_left = centre - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            vfov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
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
        let now = Instant::now();
        let bar = indicatif::ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        let img = image::ImageBuffer::from_par_fn(self.image_width, self.image_height, |x, y| {
            bar.inc(1);
            self.render_pixel(x, y, world)
        });

        let mut buf = BufWriter::new(File::create("image.png")?);
        img.write_to(&mut buf, image::ImageFormat::Png)?;
        let elapsed_time = now.elapsed();
        println!("Rendering took {} seconds", elapsed_time.as_secs_f32());
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

    #[inline]
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.centre + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    // return the ray from the cam centre to the pixel coord (i, j)
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.centre
        } else {
            self.defocus_disk_sample()
        };

        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir)
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

#[inline]
const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}
