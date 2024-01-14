use std::fmt::Debug;

use image::{Rgb, Rgba};
use crate::color::{YCbCr, Luma, Cb, Cr};

/*
    Crate of my own type and trait to handle different pixel types.
*/

/*
    Enum of the different pixel types.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelType {
    Rgb,
    Rgba,
    YCbCr,
    Luma,
    Cb,
    Cr,
}

/*
    Trait to handle different pixel types.
*/
pub trait PixelTrait : Copy + Clone + PartialEq + Eq + Debug {
    // Number of channels of the pixel type.
    const CHANNEL_COUNT: u8;
    // Type of the pixel.
    const TYPE: PixelType;

    // Get the number of channels of the pixel type.
    fn channels_count(&self) -> u8 { Self::CHANNEL_COUNT }
    // Get the channels of the pixel.
    fn channels(&self) -> Vec<u8>;

    // Create a pixel from its channels.
    fn from_channels(a: u8, b: u8, c: u8, d: u8) -> Self;

    // Create a pixel from a Rgba pixel.
    fn from_rgba(rgba: Rgba<u8>) -> Self;
    // Create a pixel from a Rgb pixel.
    fn from_rgb(rgb: Rgb<u8>) -> Self;
    // Create a pixel from a YCbCr pixel.
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self;

    // Convert a pixel to a Rgba pixel.
    fn to_rgba(&self) -> Rgba<u8>;
    // Convert a pixel to a Rgb pixel.
    fn to_rgb(&self) -> Rgb<u8>;
    // Convert a pixel to a YCbCr pixel.
    fn to_ycbcr(&self) -> YCbCr<u8>;

    // Get the type of the pixel.
    fn get_type(&self) -> PixelType { Self::TYPE }
}

/*
    Implementation of the trait for Rgb based pixels.

    Definition of the Rgb pixel type :
    - 3 channels (red, green, blue)
*/
impl PixelTrait for Rgb<u8> {
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::Rgb;

    fn channels(&self) -> Vec<u8> {
        let mut channels = Vec::new();

        for channel in self.0.iter() {
            channels.push(*channel);
        }
        return channels;
    }

    fn from_channels(a: u8, b: u8, c: u8, _: u8) -> Self { Rgb([a, b, c]) }

    fn from_rgba(rgba: Rgba<u8>) -> Self { Rgb([rgba[0], rgba[1], rgba[2]]) }
    fn from_rgb(rgb: Rgb<u8>) -> Self { rgb }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self {
        let y = ycbcr.get_y() as f32;
        let cb = ycbcr.get_cb() as f32;
        let cr = ycbcr.get_cr() as f32;

        let r = (y + 1.402*(cr - 128.0)).round() as u8;
        let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
        let b = (y + 1.772*(cb - 128.)).round() as u8;

        Rgb([r, g, b])
    }

    fn to_rgba(&self) -> Rgba<u8> { Rgba([self.0[0], self.0[1], self.0[2], 255]) }
    fn to_rgb(&self) -> Rgb<u8> { *self }
    fn to_ycbcr(&self) -> YCbCr<u8> {
        let r = self.0[0] as f32;
        let g = self.0[1] as f32;
        let b = self.0[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        YCbCr::new(y, cb, cr)
    }
}

/*
    Implementation of the trait for Rgba based pixels.

    Definition of the Rgba pixel type :
    - 4 channels (red, green, blue, alpha)
*/
impl PixelTrait for Rgba<u8> {
    const CHANNEL_COUNT: u8 = 4;
    const TYPE: PixelType = PixelType::Rgba;

    fn channels(&self) -> Vec<u8> {
        let mut channels = Vec::new();

        for channel in self.0.iter() {
            channels.push(*channel);
        }
        return channels;
    }

    fn from_channels(a: u8, b: u8, c: u8, d: u8) -> Self { Rgba([a, b, c, d]) }

    fn from_rgba(rgba: Rgba<u8>) -> Self { rgba }
    fn from_rgb(rgb: Rgb<u8>) -> Self { Rgba([rgb[0], rgb[1], rgb[2], 255]) }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self {
        let y = ycbcr.get_y() as f32;
        let cb = ycbcr.get_cb() as f32;
        let cr = ycbcr.get_cr() as f32;

        let r = (y + 1.402*(cr - 128.0)).round() as u8;
        let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
        let b = (y + 1.772*(cb - 128.)).round() as u8;

        Rgba([r, g, b, 255])
    }

    fn to_rgba(&self) -> Rgba<u8> { *self }
    fn to_rgb(&self) -> Rgb<u8> { Rgb([self.0[0], self.0[1], self.0[2]]) }
    fn to_ycbcr(&self) -> YCbCr<u8> {
        let r = self.0[0] as f32;
        let g = self.0[1] as f32;
        let b = self.0[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        YCbCr::new(y, cb, cr)
    }
}

/*
    Implementation of the trait for YCbCr based pixels.

    Definition of the YCbCr pixel type :
    - see color.rs
*/
impl PixelTrait for YCbCr<u8> {
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::YCbCr;

    fn channels(&self) -> Vec<u8> { vec![self.get_y(), self.get_cb(), self.get_cr()]}

    fn from_channels(a: u8, b: u8, c: u8, _: u8) -> Self { YCbCr::new(a, b, c) }

    fn from_rgba(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        YCbCr::new(y, cb, cr)
    }
    fn from_rgb(rgb: Rgb<u8>) -> Self {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        YCbCr::new(y, cb, cr)
    }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self { ycbcr }

    fn to_rgba(&self) -> Rgba<u8> {
        let y = self.get_y() as f32;
        let cb = self.get_cb() as f32;
        let cr = self.get_cr() as f32;

        let r = (y + 1.402*(cr - 128.0)).round() as u8;
        let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
        let b = (y + 1.772*(cb - 128.)).round() as u8;

        Rgba([r, g, b, 255])
    }
    fn to_rgb(&self) -> Rgb<u8> {
        let y = self.get_y() as f32;
        let cb = self.get_cb() as f32;
        let cr = self.get_cr() as f32;

        let r = (y + 1.402*(cr - 128.0)).round() as u8;
        let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
        let b = (y + 1.772*(cb - 128.)).round() as u8;

        Rgb([r, g, b])
    }
    fn to_ycbcr(&self) -> YCbCr<u8> { *self }
}

/*
    Implementation of the trait for Luma based pixels.

    Definition of the Luma pixel type :
    - 1 channel (luminance)
*/
impl PixelTrait for Luma<u8> {
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Luma;

    fn channels(&self) -> Vec<u8> { vec![self.get_luma()] }

    fn from_channels(a: u8, _: u8, _: u8, _: u8) -> Self { Luma::new(a) }

    fn from_rgb(rgb: Rgb<u8>) -> Self {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;

        Luma::new(y)
    }
    fn from_rgba(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;

        Luma::new(y)
    }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self {
        let y = ycbcr.get_y() as f32;

        Luma::new(y as u8)
    }

    fn to_rgba(&self) -> Rgba<u8> {
        let y = self.get_luma();

        Rgba([y, y, y, 255])
    }
    fn to_rgb(&self) -> Rgb<u8> {
        let y = self.get_luma();

        Rgb([y, y, y])
    }
    fn to_ycbcr(&self) -> YCbCr<u8> {
        let y = self.get_luma() as f32;

        YCbCr::new(y as u8, 128, 128)
    }
}

/*
    Implementation of the trait for Cb based pixels.

    Definition of the Cb pixel type :
    - 1 channel (blue-difference chroma)
*/
impl PixelTrait for Cb<u8> {
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Cb;

    fn channels(&self) -> Vec<u8> { vec![self.get_cb()] }

    fn from_channels(a: u8, _: u8, _: u8, _: u8) -> Self { Cb::new(a) }

    fn from_rgb(rgb: Rgb<u8>) -> Self {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;

        Cb::new(cb)
    }
    fn from_rgba(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;

        Cb::new(cb)
    }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self {
        let cb = ycbcr.get_cb() as f32;

        Cb::new(cb as u8)
    }

    fn to_rgba(&self) -> Rgba<u8> {
        let cb = self.get_cb() as f32;

        let r = 0;
        let g = (-0.344136*(cb - 128.0)).round() as u8;
        let b = (1.772*(cb - 128.)).round() as u8;

        Rgba([r, g, b, 255])
    }
    fn to_rgb(&self) -> Rgb<u8> {
        let cb = self.get_cb() as f32;

        let r = 0;
        let g = (-0.344136*(cb - 128.0)).round() as u8;
        let b = (1.772*(cb - 128.)).round() as u8;

        Rgb([r, g, b])
    }
    fn to_ycbcr(&self) -> YCbCr<u8> {
        let cb = self.get_cb() as f32;

        YCbCr::new(128, cb as u8, 128)
    }
}

/*
    Implementation of the trait for Cr based pixels.

    Definition of the Cr pixel type :
    - 1 channel (red-difference chroma)
*/
impl PixelTrait for Cr<u8> {
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Cr;

    fn channels(&self) -> Vec<u8> { vec![self.get_cr()] }

    fn from_channels(a: u8, _: u8, _: u8, _: u8) -> Self { Cr::new(a) }

    fn from_rgb(rgb: Rgb<u8>) -> Self {
        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        Cr::new(cr)
    }
    fn from_rgba(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

        Cr::new(cr)
    }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self {
        let cr = ycbcr.get_cr() as f32;

        Cr::new(cr as u8)
    }
    
    fn to_rgba(&self) -> Rgba<u8> {
        let cr = self.get_cr() as f32;

        let r = (1.402*(cr - 128.0)).round() as u8;
        let g = (-0.714136*(cr - 128.0)).round() as u8;
        let b = 0;

        Rgba([r, g, b, 255])
    }
    fn to_rgb(&self) -> Rgb<u8> {
        let cr = self.get_cr() as f32;

        let r = (1.402*(cr - 128.0)).round() as u8;
        let g = (-0.714136*(cr - 128.0)).round() as u8;
        let b = 0;

        Rgb([r, g, b])
    }
    fn to_ycbcr(&self) -> YCbCr<u8> {
        let cr = self.get_cr() as f32;

        YCbCr::new(128, 128, cr as u8)
    }
}