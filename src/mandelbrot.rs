extern crate image;

use utils::scale;

const MAX_ITERATIONS: u32 = 255;

#[derive(Clone, Debug)]
pub struct Mandelbrot {
    side_length: u32,
    zoom: f32,
    translation: (f32, f32),
}

// The Mandelbrot Fractal is defined within the following domains:
//  x: -2.5 .. 1.0
//  y: -1.0 .. 1.0
// (any point outside of this will always be black)
// When zooming, the ratio between the rendered x interval and y interval
// has to be kept the same to keep the image from looking stretched out.
// This ratio is: 3.5:2.0
static X_DOMAIN: (f32, f32) = (-2.5, 1.0);
static Y_DOMAIN: (f32, f32) = (-1.0, 1.0);

impl Mandelbrot {
    pub fn new(side_length: u32, zoom: f32, translation: (f32, f32)) -> Self {
        Mandelbrot {
            side_length: side_length,
            zoom: zoom,
            translation: translation,
        }
    }

    fn calculate_luma(&self, x: u32, y: u32) -> u8 {
        let side_length = self.side_length as f32;

        // Scale x and y from 0..side_length to be within (0.0, 1.0)
        let x_scaled = scale(x as f32, (0.0, side_length), (0.0, 1.0));
        let y_scaled = scale(y as f32, (0.0, side_length), (0.0, 1.0));

        // Translating means an absolute translation of the domains
        let y_domain = (Y_DOMAIN.0 + self.translation.1, Y_DOMAIN.1 + self.translation.1);
        let x_domain = (X_DOMAIN.0 + self.translation.0, X_DOMAIN.1 + self.translation.0);

        // Zooming implies a narrowing the domains
        // (meaning less of it will be visible within the frame)
        let y_domain = (y_domain.0 / self.zoom, y_domain.1 / self.zoom);
        let x_domain = (x_domain.0 / self.zoom, x_domain.1 / self.zoom);

        // Grab the point of the mandelbrot domain corresponding to (x, y) and calculate
        let x0 = scale(x_scaled, (0.0, 1.0), x_domain);
        let y0 = scale(y_scaled, (0.0, 1.0), y_domain);

        let mut x = x0;
        let mut y = y0;

        let mut i = 0;

        for t in 0..MAX_ITERATIONS {
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
