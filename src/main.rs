extern crate rayon;
extern crate image;
extern crate num;

mod mandelbrot;

use mandelbrot::Mandelbrot;
use std::env;
use std::fs::create_dir;
use rayon::prelude::*;
use std::path::Path;

static OUTPUT_DIRECTORY: &'static str = "./output";
static MAX_ITERATIONS: u32 = 255;

fn save_image(img_buf: image::GrayImage, file_name: &str) {
    let path = format!("{}/{}", OUTPUT_DIRECTORY, file_name);
    let _ = img_buf.save(Path::new(path.as_str()));
}

fn parallel_images(side_length: u32) {
    let mut iters: Vec<_> = (1..11).collect();
    iters.par_iter_mut().enumerate().for_each(|(i, _)| {
        let mut img_buf = image::ImageBuffer::new(side_length, side_length);
        let mandel = Mandelbrot::new(MAX_ITERATIONS, side_length);
        mandel.draw(&mut img_buf);

        save_image(img_buf, &format!("mandel-{}.png", i));
    });
}

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let side_length = side_length_str.parse::<u32>().unwrap();

    let _ = create_dir(OUTPUT_DIRECTORY);

    parallel_images(side_length);
}
