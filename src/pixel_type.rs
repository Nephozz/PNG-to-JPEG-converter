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
    type T: Copy + PartialEq + Debug + 'static;
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
    fn from_channels(v: &[Self::T]) -> Self;

    // Get the type of the pixel.
    fn get_type() -> PixelType { Self::TYPE }
    // Get the data type of channels.
    fn get_data_type() -> String { format!("{:?}", type_name::<Self::T>()) }
    // Get the first channel of the pixel.
    fn get_first_channel(&self) -> Self::T {
        let m = self.channels();
        *m.index((0, 0))
    }
    // Get the second channel of the pixel.
    fn get_second_channel(&self) -> Self::T {
        let m = self.channels();
        *m.index((1, 0))
    }
    // Get the third channel of the pixel.
    fn get_third_channel(&self) -> Self::T {
        let m = self.channels();
        *m.index((2, 0))
    }
    // Get the fourth channel of the pixel.
    fn get_fourth_channel(&self) -> Self::T {
        let m = self.channels();
        *m.index((3, 0))
    }

    fn from_one_channel(value: Self::T, channel: usize) -> Self {
        let p = Self::default_pixel();
        let mut m = p.channels();
        *m.index_mut((channel, 0)) = value;
        let values = m.as_slice();
        Self::from_channels(values)
    }
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

    fn from_channels(v: &[Self::T]) -> Self {
        if v.len() != 3 {
            panic!("Wrong number of channels for Rgb pixel type.");
        } 
        Rgb::new(v[0], v[1], v[2])
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

    fn from_channels(v: &[Self::T]) -> Self {
        if v.len() == 3  {
            Rgba::new(v[0], v[1], v[2], 255)
        } else if v.len() == 4 {
            Rgba::new(v[0], v[1], v[2], v[3])
        } else {
            panic!("Wrong number of channels for Rgba pixel type.");
            
        }
     }
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

    fn from_channels(v: &[Self::T]) -> Self {
        if v.len() != 3 {
            panic!("Wrong number of channels for YCbCr pixel type.");
        } 
        YCbCr::new(v[0], v[1], v[2])
     }
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

    fn from_channels(v: &[Self::T]) -> Self {
        if v.len() != 3 {
            panic!("Wrong number of channels for Yuv pixel type.");
        } 
        Yuv::new(v[0], v[1], v[2])
     }
}