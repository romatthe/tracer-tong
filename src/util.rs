use std::borrow::BorrowMut;
use std::cell::RefCell;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

thread_local! {
    static RAND: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
}

pub fn rand_float_between(min: f32, max: f32) -> f32 {
    RAND.with(|r| {
        Uniform::from(min..max).sample(&mut *r.borrow_mut())
    })
}