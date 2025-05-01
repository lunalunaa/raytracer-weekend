use std::sync::Arc;

use camera::Camera;
use ray::HittableList;
use shapes::sphere::Sphere;
use vector::Point3;

mod camera;
mod canvas;
mod color;
mod ray;
mod shapes;
mod vector;

fn main() {
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
    let mut world = HittableList::new();
    let sphere_1 = Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    let sphere_2 = Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.0));
    world.add(sphere_1);
    world.add(sphere_2);

    cam.render(&world);
}
