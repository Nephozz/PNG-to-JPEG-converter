use image::{Rgba, GenericImageView, DynamicImage, ImageBuffer};
use nalgebra::{DMatrix, Matrix2};
use crate::color::{Rgba as MyRgba, Yuv};
use crate::pixel_type::PixelTrait;
use crate::compress::Superpixel;
use crate::conversion::ConvertPixel;

/*
    Crate of my own image type with different pixel types.
*/
pub struct Image<P> where P: PixelTrait + ConvertPixel + 'static, {
    width: u32,
    height: u32,
    data: DMatrix<P>,
}

/*
    Implementation of Image for any kind of pixels.
*/
impl<P> Image<P> where P: PixelTrait + ConvertPixel {
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
    pub fn set_pixel(&mut self, x: u32, y: u32, value: &[P::T]) { self.data[(x as usize, y as usize)] = P::from_channels(value); }

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

    /*
        Convert a image to a 3 RGB channels images.
        (one channel for each image)
    */
    pub fn split(&self) -> (Image<P>, Image<P>, Image<P>) {

        //let new_image = Image::<P>::from_png(image.clone());
        
        let a_matrix = self.data.map(|x: P| P::from_one_channel(x.get_first_channel(), 0));
        let b_matrix = self.data.map(|x: P| P::from_one_channel(x.get_second_channel(), 1));
        let c_matrix = self.data.map(|x: P| P::from_one_channel(x.get_third_channel(), 2));

        let a_image = Image {width: self.width, height: self.height, data: a_matrix};
        let b_image = Image {width: self.width, height: self.height, data: b_matrix};
        let c_image = Image {width: self.width, height: self.height, data: c_matrix};

        return (a_image, b_image, c_image);
    }
}

impl Image<Yuv<f32>> {
    pub fn get_superpixels(&self, channel: usize) -> Vec<Superpixel<Yuv<f32>>> {
        let mut superpixels: Vec<Superpixel<Yuv<f32>>> = Vec::new();

        for x in (0..self.width).step_by(2) {
            for y in (0..self.height).step_by(2) {
                let slice = self.data.slice((x as usize, y as usize), ((x+2) as usize, (y+2) as usize));
                let slice_convert = slice.map(|x| x.channels()[channel]);
                let matrix = Matrix2::new(
                    slice_convert[(0, 0)], 
                    slice_convert[(0, 1)], 
                    slice_convert[(1, 0)], 
                    slice_convert[(1, 1)]
                );

                let superpixel = Superpixel::new(matrix);

                superpixels.push(superpixel);
            }
        }
        return superpixels;
    }
}
