extern crate image;

use utils::scale;

#[derive(Clone, Debug)]
pub struct Mandelbrot {
    max_iterations: u32,
    side_length: u32,
    image_no: u32,
    no_images: u32,
}

// Domains:
//  x: -2.5 .. 1.0
//  y: -1.0 .. 1.0

// When zooming, the x:y ratio (aspect ratio) has to be kept the same
// to keep the image from looking stretched out.
// This ratio is: 3.5:2.0
impl Mandelbrot {
    pub fn new(max_iters: u32, side_length: u32, image_no: u32, no_images: u32) -> Mandelbrot {
        Mandelbrot {
            max_iterations: max_iters,
            side_length: side_length,
            image_no: image_no,
            no_images: no_images,
        }
    }

    fn calculate_luma(&self, x: u32, y: u32) -> u8 {
        let image_no = self.image_no as f32;
        let side_length = self.side_length as f32;
        let no_images = self.no_images as f32;

        let total_zoom = 10.0;
        let total_translation = (-0.72, -0.135);

        // Scale x and y from 0..800 to be within (0.0, 1.0)
        let x_scaled = scale(x as f32, (0.0, side_length), (0.0, 1.0));
        let y_scaled = scale(y as f32, (0.0, side_length), (0.0, 1.0));

        // Translate and scale mandelbrot domain according to the current frame
        let y_domain = (-1.0, 1.0);
        let x_domain = (-2.5, 1.0);

        // Zooming means narrowing the domains
        let zoom_factor = total_zoom / (no_images / image_no);
        let y_domain = (y_domain.0 / zoom_factor, y_domain.1 / zoom_factor);
        let x_domain = (x_domain.0 / zoom_factor, x_domain.1 / zoom_factor);

        // Translating means an absolute translation of the domains
        let translate_x = total_translation.0 / (no_images / image_no);
        let translate_y = total_translation.1 / (no_images / image_no);
        let y_domain = (y_domain.0 + translate_y, y_domain.1 + translate_y);
        let x_domain = (x_domain.0 + translate_x, x_domain.1 + translate_x);

        // Grab the point of the mandelbrot domain corresponding to (x, y) and calculate
        let x0 = scale(x_scaled, (0.0, 1.0), x_domain);
        let y0 = scale(y_scaled, (0.0, 1.0), y_domain);

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