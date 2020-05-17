use crate::core::util::float_cmp;
use std::ops::Mul;
use crate::core::tuple::{Tuple, vector};
use std::env::current_exe;

#[derive(Clone, Debug)]
pub struct Matrix2x2 {
    data: [[f64; 2]; 2]
}

#[derive(Clone, Debug)]
pub struct Matrix3x3 {
    data: [[f64; 3]; 3]
}

#[derive(Clone, Debug)]
pub struct Matrix4x4 {
    data: [[f64; 4]; 4]
}

enum Error {
    OutOfBounds
}

impl Matrix2x2 {
    pub fn new() -> Matrix2x2 {
        Matrix2x2 {
            data: [[0.0; 2]; 2]
        }
    }

    pub fn from(data: [[f64; 2]; 2]) -> Matrix2x2 {
        Matrix2x2 {
            data
        }
    }

    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }

    pub fn at(&self, point: (usize, usize)) -> Option<f64> {
        Matrix2x2::check_bounds(point).ok().map(|_| self.data[point.0][point.1])
    }

    fn check_bounds(point: (usize, usize)) -> Result<(), Error> {
        if point.0 < 2 && point.1 < 2 {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..2 {
            for y in 0..2 {
                if !float_cmp(self.data[x][y], other.data[x][y]) {
                    return false;
                }
            }
        }

        true
    }
}

impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        Matrix3x3 {
            data: [[0.0; 3]; 3]
        }
    }

    pub fn from(data: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 {
            data
        }
    }

    pub fn sub_matrix(&self, x: usize, y: usize) -> Matrix2x2 {
        let mut sub = Matrix2x2::new();

        let mut current_x = 0;
        for w in 0..2 {
            let mut current_y = 0;

            for h in 0..2 {
                if w != x && h != y {
                    sub.data[current_x][current_y] = self.data[w][h];
                    current_y += 1;
                }
            }

            if w != x {
                current_x += 1;
            }
        }

        sub
    }

    pub fn at(&self, point: (usize, usize)) -> Option<f64> {
        Matrix3x3::check_bounds(point).ok().map(|_| self.data[point.0][point.1])
    }

    fn check_bounds(point: (usize, usize)) -> Result<(), Error> {
        if point.0 < 3 && point.1 < 3 {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if !float_cmp(self.data[x][y], other.data[x][y]) {
                    return false;
                }
            }
        }

        true
    }
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        Matrix4x4 {
            data: [[0.0; 4]; 4]
        }
    }

    pub fn from(data: [[f64; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 {
            data
        }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut res = Matrix4x4::new();

        for x in 0..4 {
            for y in 0..4 {
                res.data[x][y] = self.data[y][x]
            }
        }

        res
    }

    pub fn sub_matrix(&self, x: usize, y: usize) -> Matrix3x3 {
        let mut sub = Matrix3x3::new();



        sub
    }

    pub fn at(&self, point: (usize, usize)) -> Option<f64> {
        Matrix4x4::check_bounds(point).ok().map(|_| self.data[point.0][point.1])
    }

    fn check_bounds(point: (usize, usize)) -> Result<(), Error> {
        if point.0 < 4 && point.1 < 4 {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if !float_cmp(self.data[x][y], other.data[x][y]) {
                    return false;
                }
            }
        }

        true
    }
}

impl std::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut res = [[0.0; 4]; 4];

        for x in 0..4 {
            for y in 0..4 {
                // Multiply rows with columns
                let mut total = 0.0;

                for i in 0..4 {
                    total += self.data[x][i] * rhs.data[i][y];
                }

                res[x][y] = total;
            }
        }

        Matrix4x4 {
            data: res
        }
    }
}

impl std::ops::Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut res = [0.0; 4];

        for y in 0..4 {
            res[y] = rhs.x() * self.data[y][0]
                + rhs.y() * self.data[y][1]
                + rhs.z() * self.data[y][2]
                + rhs.w() * self.data[y][3];
        }

        Tuple::from(res)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tuple::{vector, point};

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        // Given
        let m = Matrix2x2::from([
                [-3.0, 5.0],
                [1.0, -2.0],
            ]
        );

        // Then
        assert_eq!(m.at((0, 0)).unwrap(), -3.0);
        assert_eq!(m.at((0, 1)).unwrap(), 5.0);
        assert_eq!(m.at((1, 0)).unwrap(), 1.0);
        assert_eq!(m.at((1, 1)).unwrap(), -2.0);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        // Given
        let m = Matrix3x3::from([
                [-3.0, 5.0, 0.0],
                [1.0, -2.0, -7.0],
                [0.0, 1.0, 1.0],
            ]
        );

        // Then
        assert_eq!(m.at((0, 0)).unwrap(), -3.0);
        assert_eq!(m.at((1, 1)).unwrap(), -2.0);
        assert_eq!(m.at((2, 2)).unwrap(), 1.0);
    }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        // Given
        let m = Matrix4x4::from([
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ]
        );

        // Then
        assert_eq!(m.at((0, 0)).unwrap(), 1.0);
        assert_eq!(m.at((0, 3)).unwrap(), 4.0);
        assert_eq!(m.at((1, 0)).unwrap(), 5.5);
        assert_eq!(m.at((1, 2)).unwrap(), 7.5);
        assert_eq!(m.at((2, 2)).unwrap(), 11.0);
        assert_eq!(m.at((3, 0)).unwrap(), 13.5);
        assert_eq!(m.at((3, 2)).unwrap(), 15.5);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
         // Given
        let m1 = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        // Then
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
         // Given
        let m1 = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4x4::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        // Then
        assert_ne!(m1, m2);
    }

    #[test]
    fn multiplying_two_matrices() {
        // Given
        let m1 = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix4x4::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4x4::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        // When
        let res = m1 * m2;

        // Then
        assert_eq!(res, expected);
    }

    #[test]
    fn multiplying_a_matrix_by_a_tuple() {
        // Given
        let m = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let v = Tuple::from([1.0, 2.0, 3.0, 1.0]);

        // When
        let expected = Tuple::from([18.0, 24.0, 33.0, 1.0]);
        let actual = m * v;

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        // Given
        let m = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Then
        assert_eq!(m.clone() * Matrix4x4::identity(), m)
    }

    #[test]
    fn transposing_a_matrix() {
        // Given
        let m = Matrix4x4::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix4x4::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        // When
        let actual = m.transpose();

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        // Given
        let m = Matrix4x4::identity().transpose();

        // Then
        assert_eq!(m, Matrix4x4::identity());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        // Given
        let m = Matrix2x2::from([
            [1.0, 5.0],
            [-3.0, 2.0],
        ]);

        // Then
        assert_eq!(m.determinant(), 17.0)
    }

    #[test]
    fn the_sub_matrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        // Given
        let m = Matrix3x3::from([
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0],
        ]);

        // When
        let expected = Matrix2x2::from([
            [-3.0, 2.0],
            [0.0, 6.0]
        ]);

        let actual = m.sub_matrix(0, 2);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn the_sub_matrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        // Given
        let m = Matrix4x4::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        // When
        let expected = Matrix3x3::from([
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0],
        ]);

        let actual = m.sub_matrix(2, 1);

        // Then
        assert_eq!(actual, expected);
    }
}