use crate::color::{Color};
use crate::ray::Ray;
use crate::vec::{Point, Vec3};

mod color;
mod ray;
mod vec;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit();
    let t = (unit_direction.y() + 1.0) * 0.5;

    let c1 = Color::new(1.0, 1.0, 1.0) * (1.0 - t);
    let c2 = Color::new(0.5, 0.7, 1.0) * t;

    c1 + c2
}

fn main() {

    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &origin - &(&horizontal / 2.0) - (&vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for h in (0..IMAGE_HEIGHT).rev() {

        // Print progress
        eprint!("\rScanlines remaining: {}", h);

        for w in 0..IMAGE_WIDTH {
            let u = (w as f32) / ((IMAGE_WIDTH - 1) as f32);
            let v = (h as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(origin.clone(),  (&horizontal * u) + &lower_left_corner + (&vertical * v) - &origin);

            let color = ray_color(&r);

            color::write_color(color);
        }
    }

    eprintln!("\nDone.");

}
