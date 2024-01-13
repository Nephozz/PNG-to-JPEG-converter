use image::{ImageBuffer, RgbImage};

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

fn main() {
    // a default (black) image containing Rgb values
    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    // set a central pixel to white
    *image.get_pixel_mut(5, 5) = image::Rgb([255,255,255]);

    // write it out to a file
    image.save("output/output.png").unwrap();
}