mod compress;
mod color;
mod my_image;
mod pixel_type;

use image::DynamicImage;
use std::path::Path;
use my_image::split;

//use crate::compress::Compress;

const IMG_PATH: &str = "input/input.png";
const SAVE_PATH: &str = "output/";

fn main() {
    let image: DynamicImage = image::open(Path::new(IMG_PATH)).unwrap();

    let (y_image, cb_image, cr_image) = split(image);

    //let (y_image, cb_image, cr_image) = (y_image.compress(), cb_image.compress(), cr_image.compress());

    y_image.save(&(SAVE_PATH.to_owned() + "Luma.png"));
    cb_image.save(&(SAVE_PATH.to_owned() + "Cb.png"));
    cr_image.save(&(SAVE_PATH.to_owned() + "Cr.png"));

    println!("Images saved!");
}