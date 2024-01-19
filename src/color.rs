/*
    Definition of the YCbCr color space, where:
    - Y is the luminance component
    - Cb is the blue-difference chroma component
    - Cr is the red-difference chroma component
*/

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

/*
    Definition of the YCbCr color space.
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct YCbCr<T> {
    y: Luma<T>,
    cb: Cb<T>,
    cr: Cr<T>,
}

/*
    Implementation of the YCbCr color space with u8 values.
*/
impl YCbCr<u8> {
    pub fn new(y: u8, cb: u8, cr: u8) -> Self { 
        YCbCr { 
            y: Luma::<u8>::new(y),
            cb: Cb::new(cb),
            cr: Cr::new(cr),
        } 
    }

    //pub fn YCbCr(value: [u8; 3]) -> Self { YCbCr {y: value[0], cb: value[1], cr: value[2]} }

    pub fn get_y(&self) -> u8 { self.y.get_luma() }
    pub fn get_cb(&self) -> u8 { self.cb.get_cb() }
    pub fn get_cr(&self) -> u8 { self.cr.get_cr() }
}

/*
    Definition of the YUV color space, where:
    - Y is the luminance component
    - U is the blue-difference chroma component
    - V is the red-difference chroma component
*/

/*
    Definition of single components of the YUV color space.
*/
impl Luma<i8> {
    pub fn new(luma: i8) -> Self { Luma { luma } }

    pub fn get_luma(&self) -> i8 { self.luma }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct U<T> {
    u: T,
}

impl U<i8> {
    pub fn new(u: i8) -> Self { U { u } }

    pub fn get_u(&self) -> i8 { self.u }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct V<T> {
    v: T,
}

impl V<i8> {
    pub fn new(v: i8) -> Self { V { v } }

    pub fn get_v(&self) -> i8 { self.v }
}

/*
    Definition of the YUV color space.
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Yuv<T> {
    y: Luma<T>,
    u: U<T>,
    v: V<T>,
}

/*
    Implementation of the YUV color space with u8 values.
*/
impl Yuv<i8> {
    pub fn new(y: i8, u: i8, v: i8) -> Self { 
        Yuv { 
            y: Luma::<i8>::new(y), 
            u: U::new(u), 
            v: V::new(v),
        } 
    }

    pub fn get_y(&self) -> i8 { self.y.get_luma() }
    pub fn get_u(&self) -> i8 { self.u.get_u() }
    pub fn get_v(&self) -> i8 { self.v.get_v() }
}

/*
    Definition of the RGB color space, where:
    - R is the red component
    - G is the green component
    - B is the blue component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Rgb<T> {
    r: T,
    g: T,
    b: T,
}

/*
    Implementation of the RGB color space with u8 values.
*/
impl Rgb<u8> {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Rgb { r, g, b } }

    pub fn get_red(&self) -> u8 { self.r }
    pub fn get_green(&self) -> u8 { self.g }
    pub fn get_blue(&self) -> u8 { self.b }
}

/*
    Definition of the RGBA color space, where:
    - R is the red component
    - G is the green component
    - B is the blue component
    - A is the alpha component
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Rgba<T> {
    r: T,
    g: T,
    b: T,
    a: T,
}

/*
    Implementation of the RGBA color space with u8 values.
*/
impl Rgba<u8> {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Rgba { r, g, b, a } }

    pub fn get_red(&self) -> u8 { self.r }
    pub fn get_green(&self) -> u8 { self.g }
    pub fn get_blue(&self) -> u8 { self.b }
    pub fn get_alpha(&self) -> u8 { self.a }
}