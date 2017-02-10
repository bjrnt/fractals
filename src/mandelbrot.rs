extern crate image;

#[derive(Clone, Debug)]
pub struct Mandelbrot {
    max_iterations: u32,
}

impl Mandelbrot {
    pub fn new(max_iters: u32) -> Mandelbrot {
        Mandelbrot { max_iterations: max_iters }
    }

    pub fn draw(&self, img: &mut image::GrayImage) {
        let side_length = img.width() as f32;

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let y0 = y as f32 / (side_length / 2.0) - 1.0;
            let x0 = x as f32 / (side_length / 3.5) - 2.5;

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

            *pixel = image::Luma([i as u8])
        }
    }
}