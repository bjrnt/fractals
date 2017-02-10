extern crate rayon;
extern crate image;
extern crate num;

mod mandelbrot;

use mandelbrot::Mandelbrot;
use std::env;
use std::fs::{File, create_dir};
use rayon::prelude::*;

static OUTPUT_DIRECTORY: &'static str = "./output";
static MAX_ITERATIONS: u32 = 255;

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let side_length = side_length_str.parse::<u32>().unwrap();

    let _ = create_dir(OUTPUT_DIRECTORY);

    let mut iters: Vec<_> = (1..11).collect();
    iters.par_iter_mut().enumerate().for_each(|(i, _)| {
        let path = format!("{}/mandel-{}.png", OUTPUT_DIRECTORY, i);
        let mut img_buf2 = image::ImageBuffer::new(side_length, side_length);
        let mandel = Mandelbrot::new(MAX_ITERATIONS);
        mandel.draw(&mut img_buf2);
        let f_out = &mut File::create(path).unwrap();
        let _ = image::ImageLuma8(img_buf2).save(f_out, image::PNG);
    })
}
