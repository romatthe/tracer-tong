use std::f64::EPSILON;

pub fn float_cmp(left: f64, right: f64) -> bool {
    (left - right).abs() < EPSILON
}
