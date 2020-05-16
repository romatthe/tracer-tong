use crate::core::util::float_cmp;

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn is_point(tuple: &Tuple) -> bool {
    float_cmp(tuple.w, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn is_vector(tuple: &Tuple) -> bool {
    float_cmp(tuple.w, 0.0)
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        let p = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);

        (p as f64).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();

        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot_product(&self, right: &Tuple) -> f64 {
        self.x * right.x + self.y * right.y + self.z * right.z + self.w * right.w
    }

    pub fn cross_product(&self, right: &Tuple) -> Tuple {
        vector(
            self.y * right.z - self.z * right.y,
            self.z * right.x - self.x * right.z,
            self.x * right.y - self.y * right.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_cmp(self.x, other.x)
            && float_cmp(self.y, other.y)
            && float_cmp(self.z, other.z)
            && float_cmp(self.w, other.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_one_is_a_point() {
        // Given
        let tuple = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        // Then
        assert!(is_point(&tuple));
        assert!(!is_vector(&tuple));
    }

    #[test]
    fn tuple_with_w_zero_is_a_vector() {
        // Given
        let tuple = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        // Then
        assert!(is_vector(&tuple));
        assert!(!is_point(&tuple));
    }

    #[test]
    fn point_creates_tuple_with_w_one() {
        // Given
        let tuple = point(4.3, -4.2, 3.1);

        // Then
        assert!(is_point(&tuple));
        assert!(!is_vector(&tuple));
    }

    #[test]
    fn point_creates_tuple_with_w_zero() {
        // Given
        let tuple = vector(4.3, -4.2, 3.1);

        // Then
        assert!(is_vector(&tuple));
        assert!(!is_point(&tuple));
    }

    #[test]
    fn adding_two_tuples() {
        // Given
        let left = point(3.0, -2.0, 5.0);
        let right = vector(-2.0, 3.0, 1.0);

        // Then
        assert_eq!(left + right, point(1.0, 1.0, 6.0))
    }

    #[test]
    fn subtracting_two_tuples() {
        // Given
        let left = point(3.0, 2.0, 1.0);
        let right = point(5.0, 6.0, 7.0);

        // Then
        assert_eq!(left - right, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        // Given
        let left = point(3.0, 2.0, 1.0);
        let right = vector(5.0, 6.0, 7.0);

        // Then
        assert_eq!(left - right, point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_two_vectors() {
        // Given
        let left = vector(3.0, 2.0, 1.0);
        let right = vector(5.0, 6.0, 7.0);

        // Then
        assert_eq!(left - right, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        // Given
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);

        // Then
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0))
    }

    #[test]
    fn negating_a_tuple() {
        // Given
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        // Then
        assert_eq!(
            -t,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        )
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        // Given
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        // Then
        assert_eq!(
            t * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        )
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        // Given
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        // Then
        assert_eq!(
            t * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        )
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        // Given
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        // Then
        assert_eq!(
            t / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        )
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        // Given
        let v = vector(1.0, 0.0, 0.0);

        // Then
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        // Given
        let v = vector(0.0, 1.0, 0.0);

        // Then
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        // Given
        let v = vector(0.0, 0.0, 1.0);

        // Then
        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        // Given
        let v = vector(1.0, 2.0, 3.0);

        // Then
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt())
    }

    #[test]
    fn computing_the_magnitude_of_vector_minus_1_minus_2_minus_3() {
        // Given
        let v = vector(-1.0, -2.0, -3.0);

        // Then
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt())
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        // Given
        let v = vector(1.0, 2.0, 3.0);

        let x = 1.0 / (14 as f64).sqrt();
        let y = 2.0 / (14 as f64).sqrt();
        let z = 3.0 / (14 as f64).sqrt();

        // Then
        assert_eq!(v.normalize(), vector(x, y, z))
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        // Given
        let v = vector(1.0, 2.0, 3.0);

        // When
        let norm = v.normalize();

        // Then
        assert_eq!(norm.magnitude(), 1.0)
    }

    #[test]
    fn dot_product_of_two_tuples() {
        // Given
        let l = vector(1.0, 2.0, 3.0);
        let r = vector(2.0, 3.0, 4.0);

        // Then
        assert_eq!(l.dot_product(&r), 20.0)
    }

    #[test]
    fn cross_product_of_two_tuples() {
        // Given
        let l = vector(1.0, 2.0, 3.0);
        let r = vector(2.0, 3.0, 4.0);

        // Then
        assert_eq!(l.cross_product(&r), vector(-1.0, 2.0, -1.0));
        assert_eq!(r.cross_product(&l), vector(1.0, -2.0, 1.0));
    }
}
