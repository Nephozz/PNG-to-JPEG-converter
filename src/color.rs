use image::Rgba;

/*
    Definition of the YCbCr color space, where:
    - Y is the luminance component
    - Cb is the blue-difference chroma component
    - Cr is the red-difference chroma component
*/
#[derive(Debug, Clone, Copy)]
pub struct YCbCr<T> {
    y: T,
    cb: T,
    cr: T,
}

/*
    Implementation of the YCbCr color space with u8 values.
*/
impl YCbCr<u8> {
    pub fn new_u8(y: u8, cb: u8, cr: u8) -> Self {
        YCbCr { y, cb, cr }
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_cb(&self) -> u8 {
        self.cb
    }

    pub fn get_cr(&self) -> u8 {
        self.cr
    }

    /*
        Convert a pixel from the RGBA color space to the YCbCr color space.
    */
    pub fn from_rgba(rgba: Rgba<u8>) -> Self {
        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let y = (0.257*r + 0.504*g + 0.098*b + 16.0).round() as u8;
        let cb = (-0.148*r - 0.291*g + 0.439*b + 128.0).round() as u8;
        let cr = (0.439*r - 0.368*g - 0.071*b + 128.0).round() as u8;

        YCbCr { y, cb, cr }
    }

    /*
        Convert a pixel from the YCbCr color space to the RGBA color space.
    */
    pub fn to_rgba(&self) -> Rgba<u8> {
        let y = self.y as f32;
        let cb = self.cb as f32;
        let cr = self.cr as f32;

        let r = (1.164*(y - 16.0) + 1.596*(cr - 128.0)).round() as u8;
        let g = (1.164*(y - 16.0) - 0.813*(cr - 128.0) - 0.391*(cb - 128.0)).round() as u8;
        let b = (1.164*(y - 16.0) + 2.018*(cb - 128.0)).round() as u8;

        Rgba([r, g, b, 255])
    }
}

/*
    Implementation of the YCbCr color space with i16 values.
*/
impl YCbCr<i16> {
    pub fn new_i16(y: i16, cb: i16, cr: i16) -> Self {
        YCbCr { y, cb, cr }
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }

    pub fn get_cb(&self) -> i16 {
        self.cb
    }

    pub fn get_cr(&self) -> i16 {
        self.cr
    }

    /*
        Convert a pixel from the YCbCr color space with u8 values to the YCbCr color space with i16 values.
    */
    pub fn from_u8(ycbcr: YCbCr<u8>) -> Self {
        let y = ycbcr.y as i16;
        let cb = ycbcr.cb as i16;
        let cr = ycbcr.cr as i16;

        YCbCr { y, cb, cr }
    }
}