use crate::core::color::Color;

type CanvasResult = Result<(), Err>;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

#[derive(Debug, PartialEq)]
pub enum Err {
    OutOfBounds
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color { red: 0.0, green: 0.0, blue: 0.0 }; height]; width]
        }
    }

    pub fn pixel_at(&self, width: usize, height: usize) -> Option<&Color> {
        if width < self.width && height < self.height {
            Some(&self.pixels[width][height])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, width: usize, height: usize, color: &Color) -> CanvasResult {
        if width < self.width && height < self.height {
            self.pixels[width][height] = color.clone();
            Ok(())
        } else {
            Err(Err::OutOfBounds)
        }
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
        assert!(colors
            .iter()
            .all(|&c| *c == Color { red: 0.0, green: 0.0, blue: 0.0 }));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        // Given
        let mut c = Canvas::new(10, 20);
        let red = Color { red: 1.0, blue: 0.0, green: 0.0 };

        // When
        let res = c.set_pixel(2, 3, &red);

        // Then
        assert_eq!(res, Ok(()));
        assert_eq!(c.pixel_at(2, 3), Some(&red));
    }
}
