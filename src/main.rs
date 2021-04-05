use crate::color::{Color};

mod color;
mod ray;
mod vec;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for h in (0..IMAGE_HEIGHT).rev() {

        // Print progress
        eprint!("\rScanlines remaining: {}", h);

        for w in 0..IMAGE_WIDTH {
            let color = Color::new(
                (w as f32) / ((IMAGE_WIDTH - 1) as f32),
                (h as f32) / ((IMAGE_HEIGHT - 1) as f32),
                0.25
            );

            color::write_color(color);
        }
    }

    eprintln!("\nDone.");

}
