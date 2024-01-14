/*
    Definition of the YCbCr color space, where:
    - Y is the luminance component
    - Cb is the blue-difference chroma component
    - Cr is the red-difference chroma component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct YCbCr<T> {
    y: T,
    cb: T,
    cr: T,
}

/*
    Implementation of the YCbCr color space with u8 values.
*/
impl YCbCr<u8> {
    pub fn new_u8(y: u8, cb: u8, cr: u8) -> Self { YCbCr { y, cb, cr } }

    //pub fn YCbCr(value: [u8; 3]) -> Self { YCbCr {y: value[0], cb: value[1], cr: value[2]} }

    pub fn get_y(&self) -> u8 { self.y }
    pub fn get_cb(&self) -> u8 { self.cb }
    pub fn get_cr(&self) -> u8 { self.cr }

    /*
        Convert a pixel from the YCbCr color space with i16 values to the YCbCr color space with u8 values.
    */
    pub fn from_i16(ycbcr: YCbCr<i16>) -> Self {
        let y = ycbcr.y as u8;
        let cb = ycbcr.cb as u8;
        let cr = ycbcr.cr as u8;

        YCbCr { y, cb, cr }
    }
}

/*
    Implementation of the YCbCr color space with i16 values.
*/
impl YCbCr<i16> {
    pub fn new_i16(y: i16, cb: i16, cr: i16) -> Self { YCbCr { y, cb, cr } }

    pub fn get_y(&self) -> i16 { self.y }
    pub fn get_cb(&self) -> i16 { self.cb }
    pub fn get_cr(&self) -> i16 { self.cr }

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

/*
    Definition of single components of the YCbCr color space.
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Luma<T> {
    luma: T,
}

impl Luma<u8> {
    pub fn new_u8(luma: u8) -> Self { Luma { luma } }

    pub fn get_luma(&self) -> u8 { self.luma }

    pub fn from_i16(luma: i16) -> Self {
        let luma = luma as u8;

        Luma { luma }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Cb<T> {
    cb: T,
}

impl Cb<u8> {
    pub fn new_u8(cb: u8) -> Self { Cb { cb } }

    pub fn get_cb(&self) -> u8 { self.cb }

    pub fn from_i16(cb: i16) -> Self {
        let cb = cb as u8;

        Cb { cb }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Cr<T> {
    cr: T,
}

impl Cr<u8> {
    pub fn new_u8(cr: u8) -> Self { Cr { cr } }

    pub fn get_cr(&self) -> u8 { self.cr }

    pub fn from_i16(cr: i16) -> Self {
        let cr = cr as u8;

        Cr { cr }
    }
}