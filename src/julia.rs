extern crate num;
extern crate image;

use num::complex::Complex;

#[derive(Clone, Debug)]
pub struct Julia {
    c: Complex<f32>,
    max_iterations: u32,
}

impl Julia {
    pub fn new(real: f32, imag: f32, max_iters: u32) -> Julia {
        Julia {
            c: Complex::new(real, imag),
            max_iterations: max_iters,
        }
    }

    pub fn draw(&self, img: &mut image::GrayImage) {
        let side_length = img.width() as f32;
        let scale = 2.5;

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c_y = y as f32 * scale / side_length - scale / 2.0;
            let c_x = x as f32 * scale / side_length - scale / 2.0;

            let mut z = Complex::new(c_x, c_y);

            let mut i = 0;

            for t in 0..self.max_iterations {
                if z.norm() > 2.0 {
                    break;
                }
                z = z * z + self.c;
                i = t;
            }

            *pixel = image::Luma([i as u8]);
        }
    }
}
