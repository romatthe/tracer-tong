const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for h in (0..IMAGE_HEIGHT).rev() {

        // Print progress
        eprint!("\rScanlines remaining: {}", h);

        for w in 0..IMAGE_WIDTH {
            let r = (w as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (h as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b = 0.25;

            let ir = (255.999 * r as f32) as u32;
            let ig = (255.999 * g as f32) as u32;
            let ib = (255.999 * b as f32) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");

}
