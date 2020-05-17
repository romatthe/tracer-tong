use crate::core::tuple::{Tuple, point, vector};
use crate::core::canvas::Canvas;
use crate::core::color::Color;
use std::io::Write;

pub mod core;

#[derive(Clone, Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Clone, Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity.clone();
    let vel = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: pos,
        velocity: vel
    }
}

fn main() {
    let start = point(0.0, 1.0, 0.0);
    let velocity = (vector(1.0, 1.8, 0.0).normalize()) * 11.25;
    let mut p = Projectile {
        position: start,
        velocity
    };

    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let e = Environment {
        gravity,
        wind
    };

    let mut c = Canvas::new(900, 550);

    while p.position.y() > 0.0 {
        let res = tick(e.clone(), p.clone());
        p = res;

        if p.position.y() > 0.0 {
            c.set_pixel(
                p.position.x() as usize,
                p.position.y() as usize,
                &Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0
                }
            ).unwrap();
        }

        println!("Current position: {:?}", p.position);
    }

    let ppm = c.as_ppm();

    let mut file = std::fs::File::create("out/data.txt").expect("Failed to create file");
    file.write_all(ppm.as_bytes()).expect("Failed to write to file");
}
