use std::fmt::Debug;
use std::any::type_name;
use image::{Rgb, Rgba};
use crate::color::{YCbCr, Luma, Cb, Cr, Yuv, U, V};

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
    Yuv,
    Luma,
    Cb,
    Cr,
    U,
    V
}

/*
    Trait to handle different pixel types.
*/
pub trait PixelTrait : Copy + Clone + PartialEq + Eq + Debug {
    // Type of the channels of data
    type T;
    // Number of channels of the pixel type.
    const CHANNEL_COUNT: u8;
    // Type of the pixel.
    const TYPE: PixelType;

    // Get the number of channels of the pixel type.
    fn channels_count() -> u8 { Self::CHANNEL_COUNT }
    // Get the channels of the pixel.
    fn channels(&self) -> Vec<Self::T>;

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

    fn channels(&self) -> Vec<Self::T> {
        let mut channels = Vec::new();

        for channel in self.0.iter() {
            channels.push(*channel);
        }
        return channels;
    }

    fn default_pixel() -> Self { Rgb([0, 0, 0]) }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, _: Self::T) -> Self { Rgb([a, b, c]) }
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

    fn channels(&self) -> Vec<Self::T> {
        let mut channels = Vec::new();

        for channel in self.0.iter() {
            channels.push(*channel);
        }
        return channels;
    }

    fn default_pixel() -> Self { Rgba([0, 0, 0, 255]) }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, d: Self::T) -> Self { Rgba([a, b, c, d]) }
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

    fn channels(&self) -> Vec<Self::T> { vec![self.get_y(), self.get_cb(), self.get_cr()]}

    fn default_pixel() -> Self { YCbCr::new(0, 0, 0) }

    fn from_channels(a: Self::T, b: Self::T, c:Self::T, _: Self::T) -> Self { YCbCr::new(a, b, c) }
}

/*
    Implementation of the trait for YUV based pixels.

    Definition of the YUV pixel type :
    - see color.rs
*/
impl PixelTrait for Yuv<i8> {
    type T = i8;
    const CHANNEL_COUNT: u8 = 3;
    const TYPE: PixelType = PixelType::Yuv;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_y(), self.get_u(), self.get_v()]}

    fn default_pixel() -> Self { Yuv::new(0, 0, 0) }

    fn from_channels(a: Self::T, b: Self::T, c: Self::T, _: Self::T) -> Self { Yuv::new(a, b, c) }
}

/*
    Implementation of the trait for Luma based pixels.

    Definition of the Luma pixel type :
    - 1 channel (luminance)
*/
impl PixelTrait for Luma<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Luma;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_luma()] }

    fn default_pixel() -> Self { Luma::<u8>::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { Luma::<u8>::new(a) }
}

/*
    Implementation of the trait for Cb based pixels.

    Definition of the Cb pixel type :
    - 1 channel (blue-difference chroma)
*/
impl PixelTrait for Cb<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Cb;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_cb()] }

    fn default_pixel() -> Self { Cb::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { Cb::new(a) }
}

/*
    Implementation of the trait for Cr based pixels.

    Definition of the Cr pixel type :
    - 1 channel (red-difference chroma)
*/
impl PixelTrait for Cr<u8> {
    type T = u8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Cr;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_cr()] }

    fn default_pixel() -> Self { Cr::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { Cr::new(a) }
}

impl PixelTrait for Luma<i8> {
    type T = i8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::Luma;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_luma()] }

    fn default_pixel() -> Self { Luma::<i8>::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { Luma::<i8>::new(a) }
}

/*
    Implementation of the trait for U based pixels.

    Definition of the U pixel type :
    - 1 channel (blue-difference chroma)
*/
impl PixelTrait for U<i8> {
    type T = i8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::U;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_u()] }

    fn default_pixel() -> Self { U::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { U::new(a) }
}

/*
    Implementation of the trait for V based pixels.

    Definition of the V pixel type :
    - 1 channel (red-difference chroma)
*/
impl PixelTrait for V<i8> {
    type T = i8;
    const CHANNEL_COUNT: u8 = 1;
    const TYPE: PixelType = PixelType::V;

    fn channels(&self) -> Vec<Self::T> { vec![self.get_v()] }

    fn default_pixel() -> Self { V::new(0) }

    fn from_channels(a: Self::T, _: Self::T, _: Self::T, _: Self::T) -> Self { V::new(a) }
}