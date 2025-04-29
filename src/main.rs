use std::{f64::INFINITY, rc::Rc};

use color::Color;
use ppm::PPM;
use ray::{Hittable, HittableList, Ray};
use shapes::sphere::Sphere;
use vector::{Point3, Vec3};

mod color;
mod ppm;
mod ray;
mod shapes;
mod vector;

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(hit_record) = world.hit(r, 0., INFINITY) {
        return 0.5 * (*hit_record.face_normal.normal() + Color::one());
    }

    // otherwise a gradient
    let unit_dir = r.dir.unit();
    let a = 0.5 * (unit_dir.y + 1.0);

    (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = if image_height < 1 { 1 } else { image_height };

    // world

    let mut world = HittableList::new();
    let sphere_1 = Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    let sphere_2 = Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.0));
    world.add(sphere_1);
    world.add(sphere_2);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let cam_centre = Point3::zero();

    // calculate viewport edge vectors
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // calculate pixel deltas
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate upper left pixel coordinate
    let viewport_upper_left =
        cam_centre - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut ppm = PPM::new(image_height, image_width);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_centre =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_dir = pixel_centre - cam_centre;
            let ray = Ray::new(cam_centre, ray_dir);
            let color = ray_color(&ray, &world);
            ppm.data[j][i] = color.to_rgb();
        }
    }

    ppm.export_ppm("image.ppm").unwrap();
}
