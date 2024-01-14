use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;

const IMG_PATH: &str = "output/output.png";

#[derive(Debug)]
pub struct YCbCr {
    y: u8,
    cb: u8,
    cr: u8,
}

impl YCbCr {
    pub fn new(y: u8, cb: u8, cr: u8) -> Self {
        YCbCr { y, cb, cr }
    }

    pub fn from(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let y = (0.257*r + 0.504*g + 0.098*b + 16.0).round() as u8;
        let cb = (-0.148*r - 0.291*g + 0.439*b + 128.0).round() as u8;
        let cr = (0.439*r - 0.368*g - 0.071*b + 128.0).round() as u8;

        YCbCr { y, cb, cr }
    }
}

fn divide(image: DynamicImage) -> Vec<Vec<YCbCr>> {
    let (width, height) = image.dimensions();
    let mut superpixels: Vec<Vec<YCbCr>> = Vec::new();
    
    for i in (0..width).step_by(2) {
        for j in (0..height).step_by(2) {
            let pixel1 = YCbCr::from(image.get_pixel(i, j));
            let pixel2 = YCbCr::from(image.get_pixel(i + 1, j));
            let pixel3 = YCbCr::from(image.get_pixel(i, j + 1));
            let pixel4 = YCbCr::from(image.get_pixel(i + 1, j + 1));

            superpixels.push(vec![pixel1, pixel2, pixel3, pixel4]);
        }
    }
    return superpixels;
}

fn reduce_sp(superpixel: Vec<YCbCr>) -> Vec<YCbCr> {
    let horizontal_y = superpixel[1].y - superpixel[2].y + superpixel[3].y - superpixel[0].y;
    let horizontal_cb = superpixel[1].cb - superpixel[2].cb + superpixel[3].cb - superpixel[0].cb;
    let horizontal_cr = superpixel[1].cr - superpixel[2].cr + superpixel[3].cr - superpixel[0].cr;
    let horizontal = YCbCr::new(horizontal_y, horizontal_cb, horizontal_cr);

    let vertical_y = superpixel[3].y - superpixel[0].y - superpixel[1].y + superpixel[2].y;
    let vertical_cb = superpixel[3].cb - superpixel[0].cb - superpixel[1].cb + superpixel[2].cb + superpixel[3].cb;
    let vertical_cr = superpixel[3].cr - superpixel[0].cr - superpixel[1].cr + superpixel[2].cr + superpixel[3].cr;
    let vertical = YCbCr::new(vertical_y, vertical_cb, vertical_cr);

    let diagonal_y = superpixel[0].y - superpixel[1].y - superpixel[2].y + superpixel[3].y;
    let diagonal_cb = superpixel[0].cb - superpixel[1].cb - superpixel[2].cb + superpixel[3].cb;
    let diagonal_cr = superpixel[0].cr - superpixel[1].cr - superpixel[2].cr + superpixel[3].cr;
    let diagonal = YCbCr::new(diagonal_y, diagonal_cb, diagonal_cr);

    let average_y = (superpixel[0].y + superpixel[1].y + superpixel[2].y + superpixel[3].y) / 4;
    let average_cb = (superpixel[0].cb + superpixel[1].cb + superpixel[2].cb + superpixel[3].cb) / 4;
    let average_cr = (superpixel[0].cr + superpixel[1].cr + superpixel[2].cr + superpixel[3].cr) / 4;
    let average = YCbCr::new(average_y, average_cb, average_cr);
    
    return vec![average, horizontal, vertical, diagonal];
}

fn reduce_image(superpixels: Vec<Vec<YCbCr>>) -> Vec<Vec<YCbCr>> {
    let mut reduced: Vec<Vec<YCbCr>> = Vec::new();
    for superpixel in superpixels { reduced.push(reduce_sp(superpixel)); }

    return reduced;
}

fn main() {
    let image = image::open(&Path::new(IMG_PATH)).unwrap();

    let mut superpixels = divide(image);

    while superpixels.len() > 1 {
        superpixels = reduce_image(superpixels);
    }

    println!("{:?}", superpixels);
}