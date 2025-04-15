use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use std::{path::Path, process::Command, result::Result};
use viuer::{Config, get_kitty_support, print};
mod lib;

fn main() {
    let (x, y) = viuer::terminal_size();
    let img = lib::image_handler::read_image("1.jpg".to_string());
    let padding = 2;

    let conf = viuer::Config {
        use_kitty: true,
        transparent: true,
        truecolor: true,
        x: padding,
        y: padding as i16,
        ..Default::default()
    };

    Command::new("clear").spawn().expect("command fall");

    viuer::print(&img, &conf).expect("err");
}
