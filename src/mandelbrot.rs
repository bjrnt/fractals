extern crate image;

use utils::scale;

#[derive(Clone, Debug)]
pub struct Mandelbrot {
    max_iterations: u32,
    side_length: u32,
}

// Domains:
//  x: -2.5 .. 1.0
//  y: -1.0 .. 1.0
impl Mandelbrot {
    pub fn new(max_iters: u32, side_length: u32) -> Mandelbrot {
        Mandelbrot {
            max_iterations: max_iters,
            side_length: side_length,
        }
    }

    fn calculate_luma(&self, x: u32, y: u32) -> u8 {
        // Scales from (0 .. side_length) to Mandelbrot domain
        let y0 = scale(y as f32, (0.0, self.side_length as f32), (-1.0, 1.0));
        let x0 = scale(x as f32, (0.0, self.side_length as f32), (-2.5, 1.0));

        let mut x = x0;
        let mut y = y0;

        let mut i = 0;

        for t in 0..self.max_iterations {
            if x * x + y * y >= 2.0 * 2.0 {
                break;
            }
            let x_temp = x * x - y * y + x0;
            y = 2.0 * x * y + y0;
            x = x_temp;
            i = t;
        }

        return i as u8;
    }

    pub fn draw(&self, img: &mut image::GrayImage) {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            self.draw_pixel(x, y, pixel);
        }
    }

    pub fn draw_pixel(&self, x: u32, y: u32, pixel: &mut image::Luma<u8>) {
        *pixel = image::Luma([self.calculate_luma(x, y) as u8])
    }
}