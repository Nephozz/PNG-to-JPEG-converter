use std::fmt::Debug;
use std::any::type_name;
use crate::color::{YCbCr, Yuv, Rgb, Rgba};
use nalgebra::Matrix3x1;

/*
    Crate of my own type and trait to handle different pixel types.
*/

/*
    Enum of the different pixel types.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelType {
    Rgb,
    Rgba,
    YCbCr,
    Yuv,
}

/*
    Trait to handle different pixel types.
*/
pub trait PixelTrait : Copy + Clone + PartialEq + Debug {
    // Type of the channels of data
    type T;
    // Number of channels of the pixel type.
    const CHANNEL_COUNT: u8;
    // Type of the pixel.
    const TYPE: PixelType;

    // Get the number of channels of the pixel type.
    fn channels_count() -> u8 { Self::CHANNEL_COUNT }
    // Get the channels of the pixel.
    fn channels(&self) -> Matrix3x1<Self::T>;

    // Create a pixel from default values (usually 0).
    fn default_pixel() -> Self;

    // Create a pixel from its channels.
    fn from_channels(a: Self::T, b: Self::T, c: Self::T, d: Self::T) -> Self;

    // Get the type of the pixel.
    fn get_type() -> PixelType { Self::TYPE }
    // Get the data type of channels.
    fn get_data_type() -> String { format!("{:?}", type_name::<Self::T>()) }
}

/*
    Implementation of the trait for Rgb based pixels.

    Definition of the Rgb pixel type :
    - 3 channels (red, green, blue)
*/
impl PixelTrait for Rgb<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::Rgb;

    fn channels(&self) -> Matrix3x1<Self::T> {
        let r = self.get_red();
        let g = self.get_green();
        let b = self.get_blue();
        return Matrix3x1::new(r, g, b);
    }

    fn default_pixel() -> Self { 
        Rgb::new(0, 0, 0)
     }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, _: Self::T) -> Self { 
        Rgb::new(a, b, c)
     }
}

/*
    Implementation of the trait for Rgba based pixels.

    Definition of the Rgba pixel type :
    - 4 channels (red, green, blue, alpha)
*/
impl PixelTrait for Rgba<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 4;
    const TYPE: PixelType = PixelType::Rgba;

    fn channels(&self) -> Matrix3x1<Self::T> {
        let r = self.get_red();
        let g = self.get_green();
        let b = self.get_blue();

        return Matrix3x1::new(r, g, b);
    }

    fn default_pixel() -> Self { Rgba::new(0, 0, 0, 255) }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, d: Self::T) -> Self { Rgba::new(a, b, c, d) }
}

/*
    Implementation of the trait for YCbCr based pixels.

    Definition of the YCbCr pixel type :
    - see color.rs
*/
impl PixelTrait for YCbCr<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::YCbCr;

    fn channels(&self) -> Matrix3x1<Self::T> { 
        let y = self.get_y();
        let cb = self.get_cb();
        let cr = self.get_cr();

        return Matrix3x1::new(y, cb, cr);
    }

    fn default_pixel() -> Self { YCbCr::new(0, 0, 0) }

    fn from_channels(a: Self::T, b: Self::T, c:Self::T, _: Self::T) -> Self { YCbCr::new(a, b, c) }
}

/*
    Implementation of the trait for YUV based pixels.

    Definition of the YUV pixel type :
    - see color.rs
*/
impl PixelTrait for Yuv<f32> {
    type T = f32;
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::Yuv;

    fn channels(&self) -> Matrix3x1<Self::T> { 
        let y = self.get_y();
        let u = self.get_u();
        let v = self.get_v();

        return Matrix3x1::new(y, u, v);
    }

    fn default_pixel() -> Self { Yuv::new(0., 0., 0.) }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, _: Self::T) -> Self { Yuv::new(a, b, c) }
}