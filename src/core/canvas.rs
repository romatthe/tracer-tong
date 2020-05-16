use crate::core::color::Color;

type CanvasResult = Result<(), Error>;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![
                vec![
                    Color {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0
                    };
                    height
                ];
                width
            ],
        }
    }

    pub fn pixel_at(&self, width: usize, height: usize) -> Option<&Color> {
        self.check_bounds(width, height)
            .ok()
            .map(|_| &self.pixels[width][height])
    }

    pub fn set_pixel(&mut self, width: usize, height: usize, color: &Color) -> CanvasResult {
        self.check_bounds(width, height).and_then(|_| {
            self.pixels[width][height] = color.clone();
            Ok(())
        })
    }

    fn check_bounds(&self, width: usize, height: usize) -> CanvasResult {
        if width >= self.width && height >= self.height {
            Err(Error::OutOfBounds)
        } else {
            Ok(())
        }
    }

    pub fn as_ppm(&self) -> String {
        // Header
        let mut res = format!(
            "P3\n{} {}\n255\n",
            self.width.to_string(),
            self.height.to_string()
        );

        // Data
        for y in 0..self.height {
            let mut row = String::new();

            // Checks if the length exceeds 70, starts a new line if so and adds the new value
            let mut check_len_and_add = |s: &String| {
                if row.chars().count() + s.chars().count() > 70 {
                    row.pop();
                    row.push_str("\n");
                    res.push_str(row.as_str());

                    row.clear();
                }

                row.push_str(s.as_str());
                row.push_str(" ");
            };

            // Loop over every pixel in the row
            for x in 0..self.width {
                let (x, y, z) = clamped(&self.pixels[x][y], 255);
                let (r, g, b) = (x.to_string(), y.to_string(), z.to_string());

                check_len_and_add(&r);
                check_len_and_add(&g);
                check_len_and_add(&b);
            }

            // Remove trailing space
            row.pop();
            row.push_str("\n");

            // Add the row to the result
            res.push_str(row.as_str());
        }

        res
    }
}

fn clamped(color: &Color, max_color_value: u32) -> (u32, u32, u32) {
    let r = (clamp(color.red) * (max_color_value as f64)).ceil();
    let g = (clamp(color.green) * (max_color_value as f64)).ceil();
    let b = (clamp(color.blue) * (max_color_value as f64)).ceil();

    (r as u32, g as u32, b as u32)
}

fn clamp(f: f64) -> f64 {
    if f > 1.0 {
        1.0
    } else if f < 0.0 {
        0.0
    } else {
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        // Given
        let c = Canvas::new(10, 20);

        // When
        let mut colors = vec![];

        for w in 0..10 {
            for h in 0..20 {
                colors.push(c.pixel_at(w, h).unwrap())
            }
        }

        // Then
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(colors.iter().all(|&c| *c
            == Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0
            }));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        // Given
        let mut c = Canvas::new(10, 20);
        let red = Color {
            red: 1.0,
            blue: 0.0,
            green: 0.0,
        };

        // When
        let res = c.set_pixel(2, 3, &red);

        // Then
        assert_eq!(res, Ok(()));
        assert_eq!(c.pixel_at(2, 3), Some(&red));
    }

    #[test]
    fn canvas_as_ppm_header() {
        // Given
        let c = Canvas::new(5, 3);

        // When
        let header: String = c.as_ppm().lines().take(3).collect::<Vec<&str>>().join("\n");
        let expected = "P3\n5 3\n255";

        // Then
        assert_eq!(header, expected);
    }

    #[test]
    fn constructing_ppm_pixel_data_from_canvas() {
        // Given
        let mut c = Canvas::new(5, 3);
        let c1 = Color {
            red: 1.5,
            green: 0.0,
            blue: 0.0,
        };
        let c2 = Color {
            red: 0.0,
            green: 0.5,
            blue: 0.0,
        };
        let c3 = Color {
            red: -0.5,
            green: 0.0,
            blue: 1.0,
        };

        // When
        c.set_pixel(0, 0, &c1).unwrap();
        c.set_pixel(2, 1, &c2).unwrap();
        c.set_pixel(4, 2, &c3).unwrap();

        let expected_row_1 = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let expected_row_2 = "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0";
        let expected_row_3 = "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";

        let ppm = c.as_ppm();
        let lines: Vec<&str> = ppm.lines().skip(3).take(3).collect();

        // Then
        assert_eq!(lines[0], expected_row_1);
        assert_eq!(lines[1], expected_row_2);
        assert_eq!(lines[2], expected_row_3);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        // Given
        let mut c = Canvas::new(10, 2);
        let color = Color {
            red: 1.0,
            green: 0.8,
            blue: 0.6,
        };

        for y in 0..c.height {
            for x in 0..c.width {
                c.set_pixel(x, y, &color).unwrap();
            }
        }

        // When
        let expected_row_1 = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204";
        let expected_row_2 = "153 255 204 153 255 204 153 255 204 153 255 204 153";
        let expected_row_3 = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204";
        let expected_row_4 = "153 255 204 153 255 204 153 255 204 153 255 204 153";

        let ppm = c.as_ppm();
        let lines: Vec<&str> = ppm.lines().skip(3).take(4).collect();

        // Then
        assert_eq!(lines[0], expected_row_1);
        assert_eq!(lines[1], expected_row_2);
        assert_eq!(lines[2], expected_row_3);
        assert_eq!(lines[3], expected_row_4);
    }
}
