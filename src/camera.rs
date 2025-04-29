use std::f64::INFINITY;

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
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

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
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        let mut ppm = PPM::new(self.image_height, self.image_width);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_centre = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_dir = pixel_centre - self.centre;
                let ray = Ray::new(self.centre, ray_dir);
                let color = Self::ray_color(&ray, world);
                ppm.data[j][i] = color.to_rgb();
            }
        }

        ppm.export_ppm("image.ppm").unwrap();
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(r, &Interval::new(0., INFINITY)) {
            return 0.5 * (*hit_record.face_normal.normal() + Color::one());
        }

        // otherwise a gradient
        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
    }
}
