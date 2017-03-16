extern crate rayon;
extern crate image;
extern crate num;

mod utils;
mod mandelbrot;
mod animation;

use animation::{Animation,Easing};
use std::env;
use std::fs::{create_dir, remove_dir_all};

const OUTPUT_DIRECTORY: &'static str = "./output";

fn main() {
    let side_length_str = env::args().nth(1).expect("Please provide a side length");
    let num_images_str = env::args().nth(2).expect("Please provide the number of images to render");
    let side_length = side_length_str.parse::<u32>().unwrap();
    let num_images = num_images_str.parse::<u32>().unwrap();

    let _ = remove_dir_all(OUTPUT_DIRECTORY);
    create_dir(OUTPUT_DIRECTORY).expect("Could not create output directory");

    let animation = Animation::new((0.0, 0.0),
                                   (-0.7471, -0.1488),
                                   1.0,
                                   800.0,
                                   num_images,
                                   side_length,
                                   Easing::QuadraticIn);

    animation.render_frames();
    
    if num_images > 1 {
        animation.make_gif();
    }
}
