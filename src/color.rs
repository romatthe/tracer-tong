use crate::vec::Vec3;
use crate::util::clamp;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f32;

    // Divide the color by the number of samples
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    // Write the translated [0,255] value of each color component.
    let r = (clamp(r, 0.0, 0.999) * 256.0) as u32;
    let g = (clamp(g, 0.0, 0.999) * 256.0) as u32;
    let b = (clamp(b, 0.0, 0.999) * 256.0) as u32;

    println!("{} {} {}", r, g, b);
}