use color::Color;
use ppm::PPM;
use ray::Ray;
use vector::{Point3, Vec3};

mod color;
mod ppm;
mod ray;
mod vector;

fn ray_color(r: &Ray) -> Color {
    let centre = Point3::new(0., 0., -1.);

    let t = hit_sphere(&centre, 0.5, r);

    if t >= 0. {
        // the normal vector from the centre of the sphere
        let normal = (r.at(t) - centre).unit();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.);
    }

    // otherwise a gradient
    let unit_dir = r.dir.unit();
    let a = 0.5 * (unit_dir.y + 1.0);

    (1.0 - a) * Color::one() + a * Color::new(0.5, 0.7, 1.0)
}

// returns the smallest positive t value
fn hit_sphere(centre: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *centre - r.origin;
    let a = r.dir.len_squared();
    let h = r.dir.dot(&oc);
    let c = oc.len_squared() - radius * radius;
    let discrim = h * h - a * c;

    if discrim < 0. {
        -1.
    } else {
        (h - discrim.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let cam_centre = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

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
            let color = ray_color(&ray);
            ppm.data[j][i] = color.to_rgb();
        }
    }

    ppm.export_ppm("image.ppm").unwrap();
}
