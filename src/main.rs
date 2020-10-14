mod vector;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;


use vector::{Point3, Vec3, Color};
use ray::Ray;
use hittable_list::HittableList;
use sphere::Sphere;
use hittable::Hittable;
use hittable::HitRecord;
use std::f32::INFINITY;
use camera::Camera;
use rand::Rng;


fn ray_color(ray: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth-1);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5,0.7,1.0);
}

fn main() {
    // Rng
    let mut rng = rand::thread_rng(); 

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remainin: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width-1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_height-1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            color::write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
