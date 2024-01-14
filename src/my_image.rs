use image::{Rgb, GenericImageView, DynamicImage};
use nalgebra::DMatrix;
use crate::color::{YCbCr, Luma, Cb, Cr};
use crate::pixel_type::PixelTrait;

/*
    Crate of my own image type with different pixel types.
*/
pub struct Image<P> where P: PixelTrait + 'static, {
    width: u32,
    height: u32,
    data: DMatrix<P>,
}

/*
    Implementation of Image for any kind of pixels.

    new : create a new image with the given width and height.
    from_png : create a new image from an ImageBuffer (PNG file).
    get_pixel : get the pixel at the given coordinates.
    set_pixel : set the pixel at the given coordinates.

    get_width : get the width of the image.
    get_height : get the height of the image.
*/
impl<P> Image<P> where P: PixelTrait {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: DMatrix::from_element(height as usize, width as usize, P::from_channels(0, 0, 0, 0)),
        }
    }

    pub fn from_png(image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let mut data = DMatrix::from_element(height as usize, width as usize, P::from_channels(0, 0, 0, 0));

        for i in 0..width {
            for j in 0..height {
                let pixel = P::from_rgba(image.get_pixel(i, j));
                data[(i as usize, j as usize)] = pixel;
            }
        }

        Image {width, height, data}
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> P { self.data[(x as usize, y as usize)] }
    pub fn set_pixel(&mut self, x: u32, y: u32, value: [u8; 4]) { self.data[(x as usize, y as usize)] = P::from_channels(value[0], value[1], value[2], value[3]); }

    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
}

/*
    Convert a RGB image to a YCbCr image.
*/
pub fn rgb2ycbcr(image: &Image<Rgb<u8>>) -> Image<YCbCr<u8>> {
    // to be implemented
    println!("Not implemented yet");

    return Image::new(1, 1);
}

/*
    Convert a image to a 3 RGB channels images.
    (one channel for each image)
*/
pub fn split(image: DynamicImage) -> (Image<Luma<u8>>, Image<Cb<u8>>, Image<Cr<u8>>) {
    // to be implemented
    println!("Not implemented yet");

    let y_image = Image::<_>::from_png(image.clone());
    let cb_image = Image::<_>::from_png(image.clone());
    let cr_image = Image::<_>::from_png(image);

    return (y_image, cb_image, cr_image);
}