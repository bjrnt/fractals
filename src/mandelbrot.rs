extern crate image;

use utils::scale;

#[derive(Clone, Debug)]
pub struct Mandelbrot {
    max_iterations: u32,
    side_length: u32,
    image_no: u32,
    no_images: u32,
    zoom_factor: f32,
    translate: (f32, f32),
}

// Domains:
//  x: -2.5 .. 1.0
//  y: -1.0 .. 1.0

// When zooming, the x:y ratio (aspect ratio) has to be kept the same
// to keep the image from looking stretched out.
// This ratio is: 3.5:2.0
impl Mandelbrot {
    pub fn new(max_iters: u32, side_length: u32, image_no: u32, no_images: u32) -> Mandelbrot {
        let progress = image_no as f32 / no_images as f32;
        let total_zoom = 200.0;
        let total_translation = (-0.74 * total_zoom, -0.1495 * total_zoom);

        let translation =
            (total_translation.0 * progress - total_translation.0 * 1.0 / no_images as f32,
             total_translation.1 * progress - total_translation.1 * 1.0 / no_images as f32);
        let zoom_factor = total_zoom * progress - total_zoom * 1.0 / no_images as f32 + 1.0;

        Mandelbrot {
            max_iterations: max_iters,
            side_length: side_length,
            image_no: image_no,
            no_images: no_images,
            zoom_factor: zoom_factor,
            translate: translation,
        }
    }

    fn calculate_luma(&self, x: u32, y: u32) -> u8 {
        let side_length = self.side_length as f32;

        // Scale x and y from 0..800 to be within (0.0, 1.0)
        let x_scaled = scale(x as f32, (0.0, side_length), (0.0, 1.0));
        let y_scaled = scale(y as f32, (0.0, side_length), (0.0, 1.0));

        // Translate and scale mandelbrot domain according to the current frame
        let y_domain = (-1.0, 1.0);
        let x_domain = (-2.5, 1.0);

        // Translating means an absolute translation of the domains
        let y_domain = (y_domain.0 + self.translate.1, y_domain.1 + self.translate.1);
        let x_domain = (x_domain.0 + self.translate.0, x_domain.1 + self.translate.0);

        // Zooming means narrowing the domains
        let y_domain = (y_domain.0 / self.zoom_factor, y_domain.1 / self.zoom_factor);
        let x_domain = (x_domain.0 / self.zoom_factor, x_domain.1 / self.zoom_factor);

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