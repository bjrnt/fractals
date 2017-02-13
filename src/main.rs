extern crate rayon;
extern crate image;
extern crate num;

mod utils;
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

fn parallel_images(side_length: u32, num_images: u32) {
    let mut iters: Vec<_> = (0..num_images).collect();
    iters.par_iter_mut().enumerate().for_each(|(i, _)| {
        let i = i + 1;
        let mut img_buf = image::ImageBuffer::new(side_length, side_length);
        let mandel = Mandelbrot::new(MAX_ITERATIONS, side_length, i as u32, num_images);
        mandel.draw(&mut img_buf);

        save_image(img_buf, &format!("mandel-{}.png", i));
        println!("Generate image #{}", i);
    });
}

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let num_images_str = env::args().nth(2).expect("Please provide the number of images to render");
    let side_length = side_length_str.parse::<u32>().unwrap();
    let num_images = num_images_str.parse::<u32>().unwrap();

    let _ = create_dir(OUTPUT_DIRECTORY);

    parallel_images(side_length, num_images);
}
