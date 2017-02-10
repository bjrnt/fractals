extern crate image;
extern crate num;

use mandelbrot::Mandelbrot;
mod mandelbrot;

use std::env;
use std::fs::{File, create_dir};

static OUTPUT_DIRECTORY: &'static str = "./output";
static MAX_ITERATIONS: u32 = 255;

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let side_length = side_length_str.parse::<u32>().unwrap();

    let _ = create_dir(OUTPUT_DIRECTORY);

    for i in 1..10 {
        let path = format!("{}/mandel-{}.png", OUTPUT_DIRECTORY, i);
        let mut img_buf2 = image::ImageBuffer::new(side_length, side_length);
        let mandel = Mandelbrot::new(MAX_ITERATIONS);
        mandel.draw(&mut img_buf2);
        let ref mut f_out = File::create(path).unwrap();
        let _ = image::ImageLuma8(img_buf2).save(f_out, image::PNG);
    }
}
