use std::marker::PhantomData;
use nalgebra::Matrix2;
use crate::my_image::Image;
use crate::pixel_type::PixelTrait;
use crate::conversion::Convertible;

pub struct Superpixel<P> where P: PixelTrait {
    pixels: Matrix2<i8>,
    _phantom: PhantomData<P>,
}

impl<P> Superpixel<P> where P: PixelTrait {
    pub fn new(pixels: Matrix2<i8>) -> Self {
        Superpixel {pixels, _phantom: PhantomData}
    }
}

pub trait Calculate {
    const TL: (usize, usize) = (0, 0);
    const TR: (usize, usize) = (0, 1);
    const BL: (usize, usize) = (1, 0);
    const BR: (usize, usize) = (1, 1);

    fn horizontal_diff(&self) -> i8;

    fn vertical_diff(&self) -> i8;

    fn diagonal_diff(&self) -> i8;

    fn average(&self) -> i8;
}

/*
    The four pixels of the superpixel are arranged as follows:
    
                                  _  ----------------------------  _
                                 |  |  tl  \  tr  | -tl  \ -tr  |  |
                     Average -> |  -----------------------------  | <- Vertical difference 
                               |_ |  bl  \  br  | +bl  \ +br  | _|
                               _  ----------------------------  _
                              |  | -tl  \ +tr  | +tl  \ -tr  |  |
    Horizontal difference -> |  -----------------------------  | <- Diagonal difference
                            |_ | -bl  \ +br  | -bl  \ +br  | _|
                               ----------------------------
*/
impl<P> Calculate for Superpixel<P> where P: PixelTrait {
    fn horizontal_diff(&self) -> i8 {
        let pixels = self.pixels.clone();

        -pixels[Self::TL] + pixels[Self::TR] - pixels[Self::BL] + pixels[Self::BR]
    }

    fn vertical_diff(&self) -> i8 {
        let pixels = self.pixels.clone();

        -pixels[Self::TL] - pixels[Self::TR] + pixels[Self::BL] + pixels[Self::BR]
    }

    fn diagonal_diff(&self) -> i8 {
        let pixels = self.pixels.clone();

        pixels[Self::TL] - pixels[Self::TR] - pixels[Self::BL] + pixels[Self::BR]
    }

    fn average(&self) -> i8 {
        let pixels = self.pixels.clone();

        (pixels[Self::TL] + pixels[Self::TR] + pixels[Self::BL] + pixels[Self::BR]) / 4
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

        let new_pixels = Matrix2::new(average, vertical, horizontal, diagonal);

        Superpixel::new(new_pixels)
    }
}

impl<P> Compress for Image<P> where P: PixelTrait + Convertible {
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
            todo!("Rebuild the image with the compressed superpixels");
        }

        new_image
    }
}