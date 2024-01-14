mod reduce;
mod color;
mod my_image;
mod pixel_type;

use image::DynamicImage;
use std::path::Path;
use my_image::{Image, split};

const IMG_PATH: &str = "input/input.png";
const SAVE_PATH: &str = "output/output.png";

/*
fn set_to_black(superpixels: Vec<Vec<YCbCr<u8>>>) -> Vec<Vec<YCbCr<u8>>> {
    let mut new_superpixels = superpixels.clone();
    
    for superpixel in new_superpixels.iter_mut() {
        superpixel[0] = YCbCr::new_u8(0, 0, 0);
    }
    return new_superpixels;
} 

/*
    Divide the image into superpixels of 2x2 pixels.
*/
fn divide(image: &DynamicImage) -> Vec<Vec<YCbCr<u8>>> {
    let (width, height) = image.dimensions();
    let mut superpixels: Vec<Vec<YCbCr<u8>>> = Vec::new();
    
    for i in (0..width).step_by(2) {
        for j in (0..height).step_by(2) {
            let pixel1 = YCbCr::from_rgba(image.get_pixel(i, j));
            let pixel2 = YCbCr::from_rgba(image.get_pixel(i + 1, j));
            let pixel3 = YCbCr::from_rgba(image.get_pixel(i, j + 1));
            let pixel4 = YCbCr::from_rgba(image.get_pixel(i + 1, j + 1));

            superpixels.push(vec![pixel1, pixel2, pixel3, pixel4]);
        }
    }
    return superpixels;
}

/*
    Merge the superpixels into a new image.
*/
fn merge(superpixels: Vec<Vec<YCbCr<u8>>>, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(width, height);
    let mut x = 0;
    let mut y = 0;
    
    for superpixel in superpixels {
        let pixel1 = superpixel[0].to_rgba();
        let pixel2 = superpixel[1].to_rgba();
        let pixel3 = superpixel[2].to_rgba();
        let pixel4 = superpixel[3].to_rgba();

        image.put_pixel(x, y, pixel1);
        image.put_pixel(x + 1, y, pixel2);
        image.put_pixel(x, y + 1, pixel3);
        image.put_pixel(x + 1, y + 1, pixel4);

        x += 2;

        if x >= width {
            x = 0;
            y += 2;
        }
    }
    return image;
}
*/

fn main() {
    let image: DynamicImage = image::open(Path::new(IMG_PATH)).unwrap();

    let (y_image, cb_image, cr_image) = split(image);
}