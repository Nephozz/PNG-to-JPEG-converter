use image::{Rgba, GenericImageView, DynamicImage, ImageBuffer};
use nalgebra::{DMatrix, Matrix2};
use crate::color::{Luma, Cb, Cr};
use crate::pixel_type::PixelTrait;
use crate::compress::Superpixel;

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
*/
impl<P> Image<P> where P: PixelTrait {
    /*
        Create a new image with the given width and height.
    */
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            data: DMatrix::from_element(height as usize, width as usize, P::default_pixel()),
        }
    }

    /*
        Create a new image from an ImageBuffer (PNG file).
    */
    pub fn from_png(image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let mut data = DMatrix::from_element(height as usize, width as usize, P::default_pixel());

        for x in 0..width {
            for y in 0..height {
                let pixel = P::from_rgba(image.get_pixel(x, y));
                data[(x as usize, y as usize)] = pixel;
            }
        }

        Image {width, height, data}
    }

    /*
        Get and set pixels.
    */
    pub fn get_pixel(&self, x: u32, y: u32) -> P { self.data[(x as usize, y as usize)] }
    pub fn set_pixel(&mut self, x: u32, y: u32, value: [P::T; 4]) { self.data[(x as usize, y as usize)] = P::from_channels(value[0], value[1], value[2], value[3]); }

    /*
        Get the width and height of the image.
    */
    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }

    /*
        Save the image to a PNG file.
    */
    pub fn save(&self, path: &str) {
        let mut image = ImageBuffer::<Rgba<u8>,Vec<u8>>::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.get_pixel(x, y).to_rgba();
                image.put_pixel(x, y, pixel);
            }
        }

        image.save(path).unwrap();
    }

    pub fn get_superpixels(&self) -> Vec<Superpixel<P>> {
        let mut superpixels: Vec<Superpixel<P>> = Vec::new();

        if P::CHANNEL_COUNT != 1 {
            panic!("The pixel type is not single channel");
        } else if P::get_data_type() != "i8" {
            panic!("The pixel type is not i8, can't perform calculations");
        } else {
            for x in (0..self.width).step_by(2) {
                for y in (0..self.height).step_by(2) {
                    let slice = self.data.slice((0, 0), (2, 2));
                    //slice.map()
                    let matrix = Matrix2::new(slice[(0, 0)], slice[(0, 1)], slice[(1, 0)], slice[(1, 1)]);
                    let superpixel = Superpixel::new(matrix);
    
                    superpixels.push(superpixel);
                }
            }
        }
        return superpixels;
    }
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

fn convert_vec(vec: Vec<u8>) -> Vec<i16> {
    let mut new_vec = Vec::new();

    for elt in vec.iter() {
        new_vec.push(*elt as i16);
    }

    new_vec
}