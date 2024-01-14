use std::marker::PhantomData;
use crate::my_image::Image;
use crate::pixel_type::PixelTrait;

pub struct Superpixel<P> where P: PixelTrait {
    pixels: Vec<u8>,
    _phantom: PhantomData<P>,
}

impl<P> Superpixel<P> where P: PixelTrait {
    pub fn new(pixels: Vec<u8>) -> Self {
        Superpixel {pixels, _phantom: PhantomData}
    }
}

pub trait Calculate {
    fn horizontal_diff(&self) -> u8;

    fn vertical_diff(&self) -> u8;

    fn diagonal_diff(&self) -> u8;

    fn average(&self) -> u8;
}

impl<P> Calculate for Superpixel<P> where P: PixelTrait {
    fn horizontal_diff(&self) -> u8 {
        let pixels = convert_vec(self.pixels.clone());

        (pixels[1] - pixels[2] + pixels[3] - pixels[0]) as u8
    }

    fn vertical_diff(&self) -> u8 {
        let pixels = convert_vec(self.pixels.clone());

        (pixels[2] - pixels[0] + pixels[3] - pixels[1]) as u8
    }

    fn diagonal_diff(&self) -> u8 {
        let pixels = convert_vec(self.pixels.clone());

        (pixels[0] - pixels[1] - pixels[2] + pixels[3]) as u8
    }

    fn average(&self) -> u8 {
        let pixels = convert_vec(self.pixels.clone());

        ((pixels[0] + pixels[1] + pixels[2] + pixels[3]) / 4) as u8
    }
}

pub trait Compress {
    fn compress(&self) -> Self;
}

impl<P> Compress for Superpixel<P> where P: PixelTrait {
    /*
        Return the new superpixel with the compressed values.
        The first (top left) pixel is the average of the four pixels of the superpixel.
        The second (top right) pixel is the vertical difference between the four pixels of the superpixel.
        The third (bottom left) pixel is the horizontal difference between the four pixels of the superpixel.
        The fourth (bottom right) pixel is the diagonal difference between the four pixels of the superpixel.
    */
    fn compress(&self) -> Self {
        let horizontal = self.horizontal_diff();
        let vertical = self.vertical_diff();
        let diagonal = self.diagonal_diff();
        let average = self.average();

        Superpixel::new(vec![average, vertical, horizontal, diagonal])
    }
}

impl<P> Compress for Image<P> where P: PixelTrait {
    fn compress(&self) -> Self {
        let mut new_image = Image::new(self.get_width(), self.get_height());
        let superpixels = self.get_superpixels();

        // Compress each superpixel
        let mut compressed_superpixels: Vec<Superpixel<P>> = Vec::new();
        for superpixel in superpixels {
            let compressed = superpixel.compress();
            compressed_superpixels.push(compressed);
        }
        
        // Rebuid the image with the compressed superpixels
        for (i, superpixel) in compressed_superpixels.iter().enumerate() {
            let pixels = superpixel.pixels.clone();
            let x = (i % (self.get_width() as usize / 2)) * 2;
            let y = (i / (self.get_width() as usize / 2)) * 2;

            new_image.set_pixel(x as u32, y as u32, [pixels[0], pixels[0], pixels[0], 255]);
            new_image.set_pixel(x as u32 + 1, y as u32, [pixels[1], pixels[1], pixels[1], 255]);
            new_image.set_pixel(x as u32, y as u32 + 1, [pixels[2], pixels[2], pixels[2], 255]);
            new_image.set_pixel(x as u32 + 1, y as u32 + 1, [pixels[3], pixels[3], pixels[3], 255]);
        }

        new_image
    }
}

fn convert_vec(vec: Vec<u8>) -> Vec<i16> {
    let mut new_vec = Vec::new();

    for elt in vec.iter() {
        new_vec.push(*elt as i16);
    }

    new_vec
}