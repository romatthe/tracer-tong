use crate::core::util::float_cmp;
use std::ops::{Add, Sub, Mul};

#[derive(Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_cmp(self.red, other.red)
            && float_cmp(self.green, other.green)
            && float_cmp(self.blue, other.blue)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        // Given
        let c = Color   {
            red: -0.5,
            green: 0.4,
            blue: 1.7
        };

        // Then
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        // Given
        let c1 = Color { red: 0.9, green: 0.6, blue: 0.75 };
        let c2 = Color { red: 0.7, green: 0.1, blue: 0.25 };

        // Then
        assert_eq!(c1 + c2, Color { red: 1.6, green: 0.7, blue: 1.0 })
    }

    #[test]
    fn subtracting_colors() {
        // Given
        let c1 = Color { red: 0.9, green: 0.6, blue: 0.75 };
        let c2 = Color { red: 0.7, green: 0.1, blue: 0.25 };

        // Then
        assert_eq!(c1 - c2, Color { red: 0.2, green: 0.5, blue: 0.5 })
    }

    #[test]
    fn multiplying_colors_by_scalar() {
        // Given
        let c = Color { red: 0.2, green: 0.3, blue: 0.4 };

        // Then
        assert_eq!(c * 2.0, Color { red: 0.4, green: 0.6, blue: 0.8 })
    }

    #[test]
    fn multiplying_colors() {
        // Given
        let c1 = Color { red: 1.0, green: 0.2, blue: 0.4 };
        let c2 = Color { red: 0.9, green: 1.0, blue: 0.1 };

        // Then
        assert_eq!(c1 * c2, Color { red: 0.9, green: 0.2, blue: 0.04 })
    }
}

