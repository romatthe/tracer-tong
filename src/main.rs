use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec::{Point, Vec3};
use crate::scene::{Hit, SceneObject, Sphere, Camera};

mod color;
mod ray;
mod scene;
mod util;
mod vec;

fn ray_color(r: &Ray, world: &dyn SceneObject, depth: u32) -> Color {
    let mut hit = Hit::default();

    // If we've exceeded the ray bounce limit, no more light  is gathered
    if depth <= 0 {
        return Color::empty();
    }

    if world.hit(r, 0.0, f32::INFINITY, &mut hit) {
        let target = hit.normal() + hit.point() + Vec3::random_in_unit_sphere();
        let ray = Ray::new(hit.point().clone(), target - hit.point());
        return ray_color(&ray, world, depth - 1) * 0.5;
    }

    let unit_direction = r.direction().unit();
    let t = (unit_direction.y() + 1.0) * 0.5;

    let c1 = Color::new(1.0, 1.0, 1.0) * (1.0 - t);
    let c2 = Color::new(0.5, 0.7, 1.0) * t;

    c1 + c2
}

fn hit_sphere(center: &Point, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center;

    let a = ray.direction().len_squared();
    let b = oc.dot(ray.direction());
    let c = oc.len_squared() - (radius * radius);

    let discriminant = (b * b) - (a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / a
    }
}

fn main() {

    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let world: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for h in (0..IMAGE_HEIGHT).rev() {

        // Print progress
        eprint!("\rScanlines remaining: {}", h);

        for w in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (w as f32 + util::rand_float()) / (IMAGE_WIDTH - 1) as f32;
                let v = (h as f32 + util::rand_float()) / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");

}
