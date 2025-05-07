use std::sync::Arc;

use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian, Material, Metal};
use ray::{BVHNode, HittableList};
use shapes::sphere::Sphere;
use vector::{Point3, Vec3};

mod aabb;
mod camera;
mod color;
mod material;
mod ray;
mod shapes;
mod vector;

use anyhow::Result;

fn main() -> Result<()> {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let sphere = Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(ground_material),
    );

    world.add(Arc::new(sphere));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random_range(0.0..1.0);
            let centre = Point3::new(
                a as f64 + 0.9 * rand::random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rand::random_range(0.0..1.0),
            );

            if (centre - Vec3::new(4.0, 0.2, 0.)).len() > 0.9 {
                let sphere_meterial: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_range(0.0, 1.0) * Color::random_range(0.0, 1.0);
                    sphere_meterial = Arc::new(Lambertian::new(albedo));
                    let sphere = Sphere::new(centre, 0.2, sphere_meterial);
                    world.add(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal

                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    sphere_meterial = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(centre, 0.2, sphere_meterial);
                    world.add(Arc::new(sphere));
                } else {
                    sphere_meterial = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(centre, 0.2, sphere_meterial);
                    world.add(Arc::new(sphere));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1., 0.),
        1.,
        material3,
    )));

    let bvh_node = BVHNode::from_hittable_list(world);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh_node));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_bounce_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_bounce_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    cam.render(&world)?;

    Ok(())
}
