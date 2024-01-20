mod compress;
mod color;
mod my_image;
mod pixel_type;
mod conversion;

use image::DynamicImage;
use std::path::Path;
use crate::conversion::ConvertImage;
use crate::my_image::Image;
use crate::color::{YCbCr, Rgb, Yuv};
//use my_image::split;

//use crate::compress::Compress;

const IMG_PATH: &str = "input/input.png";
const SAVE_PATH: &str = "output/";

fn main() {
    let image: DynamicImage = image::open(Path::new(IMG_PATH)).unwrap();
    let imp_image: Image<Rgb<u8>> = Image::<Rgb<u8>>::from_png(image);

    let (r_image, g_image, b_image) = Image::split(&imp_image);

    let ycbcr_image = imp_image.to_yuv();
    let (y_image, cb_image, cr_image) = Image::split(&ycbcr_image);

    r_image.save(&(SAVE_PATH.to_owned() + "Red.png"));
    g_image.save(&(SAVE_PATH.to_owned() + "Green.png"));
    b_image.save(&(SAVE_PATH.to_owned() + "Blue.png"));

    y_image.save(&(SAVE_PATH.to_owned() + "Luma.png"));
    cb_image.save(&(SAVE_PATH.to_owned() + "Cb.png"));
    cr_image.save(&(SAVE_PATH.to_owned() + "Cr.png"));

    println!("Images saved!");
}