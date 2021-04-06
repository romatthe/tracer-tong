use std::cell::RefCell;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

thread_local! {
    static RAND: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
}

pub fn rand_float() -> f32 {
    rand::random()
}

pub fn rand_float_between(min: f32, max: f32) -> f32 {
    RAND.with(|r| {
        Uniform::from(min..max).sample(&mut *r.borrow_mut())
    })
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}