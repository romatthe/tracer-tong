use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec::Point;
use crate::scene::{Hit, SceneObject, Sphere, Camera};

mod color;
mod ray;
mod scene;
mod util;
mod vec;

fn ray_color(r: &Ray, world: &dyn SceneObject) -> Color {
    let mut hit = Hit::default();

    if world.hit(r, 0.0, f32::INFINITY, &mut hit) {
        return (Color::new(1.0, 1.0, 1.0) + hit.normal()) * 0.5;
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

    // World
    let world: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    // let viewport_height = 2.0;
    // let viewport_width = ASPECT_RATIO * viewport_height;
    // let focal_length = 1.0;
    let camera = Camera::new();

    // let origin = Point::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner = &origin - &(&horizontal / 2.0) - (&vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

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
                pixel_color += ray_color(&ray, &world);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");

}
