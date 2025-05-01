use crate::{
    color::Color,
    ppm::PPM,
    ray::{Hittable, Interval, Ray},
    vector::{Point3, Vec3},
};

#[allow(unused)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    pub samples_per_pixel: usize,
    pixel_samples_scale: f64,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
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
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        let mut ppm = PPM::new(self.image_height, self.image_width);
        let bar = indicatif::ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let mut color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color += Self::ray_color(&r, world);
                }
                ppm.data[j][i] = (color * self.pixel_samples_scale).to_rgb();
            }
        }

        ppm.export_ppm("image.ppm").unwrap();
    }

    // return a pair within [-0.5, 0.5], [-0.5, 0.5] range
    fn sample_square() -> (f64, f64) {
        (
            rand::random_range(-0.5..=0.5),
            rand::random_range(-0.5..=0.5),
        )
    }

    // return the ray from the cam centre to the pixel coord (i, j)
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);
        let ray_dir = pixel_sample - self.centre;

        Ray::new(self.centre, ray_dir)
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(r, &Interval::new(0., f64::INFINITY)) {
            return 0.5 * (*hit_record.face_normal.normal() + Color::one());
        }

        // otherwise a gradient
        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
    }
}
