extern crate rayon;
extern crate image;
extern crate num;

mod utils;
mod mandelbrot;

use mandelbrot::Mandelbrot;
use std::env;
use std::fs::{create_dir, remove_dir_all, canonicalize};
use rayon::prelude::*;
use std::path::Path;
use std::process::Command;

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
        save_image(img_buf, &format!("mandel-{:03}.png", i));
        println!("Completed image #{}", i);
    });
}

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let num_images_str = env::args().nth(2).expect("Please provide the number of images to render");
    let side_length = side_length_str.parse::<u32>().unwrap();
    let num_images = num_images_str.parse::<u32>().unwrap();

    let _ = remove_dir_all(OUTPUT_DIRECTORY);
    create_dir(OUTPUT_DIRECTORY).expect("Could not create output directory");

    parallel_images(side_length, num_images);

    if num_images > 1 {
        let output_path = canonicalize(OUTPUT_DIRECTORY)
            .expect("Could not get absolute path from output directory");

        Command::new("/bin/sh")
            .arg("-c")
            .arg("/usr/local/bin/convert -delay 7 -loop 0 -antialias -resize 500x500 *.png animation.gif")
            .current_dir(output_path)
            .spawn()
            .expect("Could not convert result into gif");
    }
}
