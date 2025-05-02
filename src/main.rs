use std::sync::Arc;

use camera::Camera;
use color::Color;
use material::{Lambertian, Metal};
use ray::HittableList;
use shapes::sphere::Sphere;
use vector::Point3;

mod camera;
mod color;
mod material;
mod ray;
mod shapes;
mod vector;

use anyhow::Result;

fn main() -> Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_bounce_depth = 50;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_bounce_depth,
    );

    // world

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();
    let ground = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.0,
        material_ground,
    ));
    let sphere_left = Arc::new(Sphere::new(Point3::new(-1.0, 0., -1.0), 0.5, material_left));
    let sphere_centre = Arc::new(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    let sphere_right = Arc::new(Sphere::new(Point3::new(1.0, 0., -1.0), 0.5, material_right));

    world.add(ground);
    world.add(sphere_left);
    world.add(sphere_centre);
    world.add(sphere_right);

    cam.render(&world)?;

    Ok(())
}
