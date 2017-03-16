extern crate rayon;
extern crate image;

use rayon::prelude::*;
use mandelbrot::Mandelbrot;
use OUTPUT_DIRECTORY;
use std::path::Path;
use std::fs::canonicalize;
use std::process::Command;

pub enum Easing {
    Linear,
    QuadraticIn,
}

pub struct Animation {
    start_translation: (f32, f32),
    end_translation: (f32, f32),
    start_zoom: f32,
    end_zoom: f32,
    num_images: u32,
    side_length: u32,
    easing: Easing
}

fn save_image(img_buf: image::GrayImage, file_name: &str) {
    let path = format!("{}/{}", OUTPUT_DIRECTORY, file_name);
    let _ = img_buf.save(Path::new(path.as_str()));
}

impl Animation {
    pub fn new(st: (f32, f32), et: (f32, f32), sz: f32, ez: f32, ni: u32, sl: u32, easing: Easing) -> Self {
        Animation {
            start_translation: st,
            end_translation: et,
            start_zoom: sz,
            end_zoom: ez,
            num_images: ni,
            side_length: sl,
            easing: easing
        }
    }

    fn render_frame(&self, frame_no: u32) {
        // calculate percentage complete based on which frame we're on
        let progress = frame_no as f32 / self.num_images as f32;
        // this essentially creates a QuadraticIn tween
        let progress = progress * progress;

        let zoom = self.start_zoom + progress * (self.end_zoom - self.start_zoom);

        let (start_x, start_y) = self.start_translation;
        let (end_x, end_y) = self.end_translation;
        let translate_x = (start_x + progress * (end_x - start_x)) * zoom;
        let translate_y = (start_y + progress * (end_y - start_y)) * zoom;

        let mut img_buf = image::ImageBuffer::new(self.side_length, self.side_length);
        let mandel = Mandelbrot::new(self.side_length,
                                     zoom,
                                     (translate_x, translate_y));
        mandel.draw(&mut img_buf);
        save_image(img_buf, &format!("mandel-{:03}.png", frame_no));
    }

    pub fn render_frames(&self) {
        let frames: Vec<_> = (1..self.num_images + 1).collect();
        frames.par_iter().for_each(|i| {
            self.render_frame(*i as u32);
        });
    }

    pub fn make_gif(&self) {
        let output_path = canonicalize(OUTPUT_DIRECTORY)
            .expect("Could not get absolute path from output directory");

        Command::new("/bin/sh")
            .arg("-c")
            .arg("/usr/local/bin/convert -delay 7 -loop 0 -antialias -resize 500x500 *.png \
                  animation.gif")
            .current_dir(output_path)
            .spawn()
            .expect("Could not convert result into gif");
    }
}