use image::{Rgba, GenericImageView, DynamicImage, ImageBuffer};
use nalgebra::{DMatrix, Matrix2};
use crate::color::{YCbCr, Rgba as MyRgba, Yuv};
use crate::pixel_type::PixelTrait;
use crate::compress::Superpixel;
use crate::conversion::Convertible;

/*
    Crate of my own image type with different pixel types.
*/
pub struct Image<P> where P: PixelTrait + Convertible + 'static, {
    width: u32,
    height: u32,
    data: DMatrix<P>,
}

/*
    Implementation of Image for any kind of pixels.
*/
impl<P> Image<P> where P: PixelTrait + Convertible {
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
                let img_pixel: Rgba<u8> = image.get_pixel(x, y);
                let pixel = MyRgba::new(img_pixel.0[0], img_pixel.0[1], img_pixel.0[2], img_pixel.0[3]);
                let new_pixel = P::from_rgba(pixel);
                data[(x as usize, y as usize)] = new_pixel;
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
                let pixel: MyRgba<u8> = self.get_pixel(x, y).to_rgba();
                let new_pixel = Rgba([pixel.get_red(), pixel.get_green(), pixel.get_blue(), pixel.get_alpha()]);
                image.put_pixel(x, y, new_pixel);
            }
        }

        image.save(path).unwrap();
    }

    pub fn get_superpixels(&self) -> Vec<Superpixel<P>> {
        let mut superpixels: Vec<Superpixel<P>> = Vec::new();

        if P::CHANNEL_COUNT != 1 {
            panic!("The pixel type is not single channel");
        } else if P::get_data_type() != "f32" {
            panic!("The pixel type is not f32, can't perform calculations");
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
pub fn split(image: DynamicImage) -> (Image<YCbCr<u8>>, Image<YCbCr<u8>>, Image<YCbCr<u8>>) {

    let y_image = Image::<YCbCr<u8>>::from_png(image.clone());
    y_image.data.map(|x| x.get_y());
    let cb_image = Image::<YCbCr<u8>>::from_png(image.clone());
    cb_image.data.map(|x| x.get_cb());
    let cr_image = Image::<YCbCr<u8>>::from_png(image);
    cr_image.data.map(|x| x.get_cr());

    return (y_image, cb_image, cr_image);
}
/*


fn convert_vec(vec: Vec<u8>) -> Vec<i16> {
    let mut new_vec = Vec::new();

    for elt in vec.iter() {
        new_vec.push(*elt as i16);
    }

    new_vec
}

*/
