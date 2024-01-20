use std::fmt::Debug;
use nalgebra::Matrix3x1;

/*
    Definition of the YCbCr color space, where:
    - Y is the luminance component
    - Cb is the blue-difference chroma component
    - Cr is the red-difference chroma component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct YCbCr<T: Copy + PartialEq + Debug + 'static> {
    channel: Matrix3x1<T>,
}

/*
    Implementation of the YCbCr color space with u8 values.
*/
impl YCbCr<u8> {
    pub fn new(y: u8, cb: u8, cr: u8) -> Self { 
        YCbCr { 
            channel: Matrix3x1::new(y, cb, cr)
        } 
    }

    //pub fn YCbCr(value: [u8; 3]) -> Self { YCbCr {y: value[0], cb: value[1], cr: value[2]} }

    pub fn get_y(&self) -> u8 { self.channel[0] }
    pub fn get_cb(&self) -> u8 { self.channel[1] }
    pub fn get_cr(&self) -> u8 { self.channel[2] }
}

/*
    Definition of the YUV color space, where:
    - Y is the luminance component
    - U is the blue-difference chroma component
    - V is the red-difference chroma component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Yuv<T: Copy + Debug + 'static> {
    channels: Matrix3x1<T>,
}

/*
    Implementation of the YUV color space with u8 values.
*/
impl Yuv<f32> {
    pub fn new(y: f32, u: f32, v: f32) -> Self { 
        Yuv { 
            channels: Matrix3x1::new(y, u, v),
        } 
    }

    pub fn get_y(&self) -> f32 { self.channels[0] }
    pub fn get_u(&self) -> f32 { self.channels[1] }
    pub fn get_v(&self) -> f32 { self.channels[2] }
}

/*
    Definition of the RGB color space, where:
    - R is the red component
    - G is the green component
    - B is the blue component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Rgb<T: Copy + Debug + 'static> {
    channels: Matrix3x1<T>,
}

/*
    Implementation of the RGB color space with u8 values.
*/
impl Rgb<u8> {
    pub fn new(r: u8, g: u8, b: u8) -> Self { 
        Rgb { channels: Matrix3x1::new(r, g, b) }
     }

    pub fn get_red(&self) -> u8 { self.channels[0] }
    pub fn get_green(&self) -> u8 { self.channels[1] }
    pub fn get_blue(&self) -> u8 { self.channels[2] }
}

/*
    Definition of the RGBA color space, where:
    - R is the red component
    - G is the green component
    - B is the blue component
    - A is the alpha component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Rgba<T: Copy + Debug + 'static> {
    channels: Matrix3x1<T>,
    a: T,
}

/*
    Implementation of the RGBA color space with u8 values.
*/
impl Rgba<u8> {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Rgba { 
        channels: Matrix3x1::new(r, g, b), a } }

    pub fn get_red(&self) -> u8 { self.channels[0] }
    pub fn get_green(&self) -> u8 { self.channels[1] }
    pub fn get_blue(&self) -> u8 { self.channels[2] }
    pub fn get_alpha(&self) -> u8 { self.a }
}