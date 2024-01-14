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
    pub fn new(y: u8, cb: u8, cr: u8) -> Self { YCbCr { y, cb, cr } }

    //pub fn YCbCr(value: [u8; 3]) -> Self { YCbCr {y: value[0], cb: value[1], cr: value[2]} }

    pub fn get_y(&self) -> u8 { self.y }
    pub fn get_cb(&self) -> u8 { self.cb }
    pub fn get_cr(&self) -> u8 { self.cr }
}
/*
    Definition of single components of the YCbCr color space.
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Luma<T> {
    luma: T,
}

impl Luma<u8> {
    pub fn new(luma: u8) -> Self { Luma { luma } }

    pub fn get_luma(&self) -> u8 { self.luma }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Cb<T> {
    cb: T,
}

impl Cb<u8> {
    pub fn new(cb: u8) -> Self { Cb { cb } }

    pub fn get_cb(&self) -> u8 { self.cb }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Cr<T> {
    cr: T,
}

impl Cr<u8> {
    pub fn new(cr: u8) -> Self { Cr { cr } }

    pub fn get_cr(&self) -> u8 { self.cr }
}