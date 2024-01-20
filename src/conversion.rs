use crate::color::{YCbCr, Rgb, Rgba, Yuv};
use nalgebra::{Matrix3, Matrix3x1};
use crate::pixel_type::PixelTrait;
use crate::my_image::Image;

pub fn rgb2rgba(rgb: Rgb<u8>) -> Rgba<u8> {
    Rgba::<u8>::new(rgb.get_red(), rgb.get_green(), rgb.get_blue(), 255)
}

pub fn rgb2ycbcr(rgb: Rgb<u8>) -> YCbCr<u8> {
    let m = Matrix3::new(
        0.299, 0.587, 0.114,
        -0.168736, -0.331264, 0.5,
        0.5, -0.418688, -0.081312
    );

    let mut res = m*rgb.channels().map(|x| x as f64);
    res += Matrix3x1::new(0., 128., 128.);

    YCbCr::new(res[0] as u8, res[1] as u8, res[2] as u8)
}

pub fn rgb2yuv(rgb: Rgb<u8>) -> Yuv<f32> {
    let m = Matrix3::new(
        0.2126, 0.7152, 0.0722,
        -0.09991, -0.33609, 0.436,
        0.615, -0.55861, -0.05639
    );

    let res = m*rgb.channels().map(|x| x as f64);

    Yuv::new(res[0] as f32, res[1] as f32, res[2] as f32)
}

pub fn rgba2rgb(rgba: Rgba<u8>) -> Rgb<u8> {
    Rgb::<u8>::new(rgba.get_red(), rgba.get_green(), rgba.get_blue())
}

pub fn rgba2ycbcr(rgba: Rgba<u8>) -> YCbCr<u8> {
    let m = Matrix3::new(
        0.299, 0.587, 0.114,
        -0.168736, -0.331264, 0.5,
        0.5, -0.418688, -0.081312
    );

    let mut res = m*rgba.channels().map(|x| x as f64);
    res += Matrix3x1::new(0., 128., 128.);

    YCbCr::new(res[0] as u8, res[1] as u8, res[2] as u8)
}

pub fn rgba2yuv(rgba: Rgba<u8>) -> Yuv<f32> {
    let m = Matrix3::new(
        0.2126, 0.7152, 0.0722,
        -0.09991, -0.33609, 0.436,
        0.615, -0.55861, -0.05639
    );

    let res = m*rgba.channels().map(|x| x as f32);

    Yuv::new(res[0] as f32, res[1] as f32, res[2] as f32)
}

pub fn ycbcr2rgb(ycbcr: YCbCr<u8>) -> Rgb<u8> {
    let y = ycbcr.get_y() as f32;
    let cb = ycbcr.get_cb() as f32;
    let cr = ycbcr.get_cr() as f32;

    let r = (y + 1.402*(cr - 128.0)).round() as u8;
    let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
    let b = (y + 1.772*(cb - 128.)).round() as u8;

    Rgb::<u8>::new(r, g, b)
}

pub fn ycbcr2rgba(ycbcr: YCbCr<u8>) -> Rgba<u8> {
    let y = ycbcr.get_y() as f32;
    let cb = ycbcr.get_cb() as f32;
    let cr = ycbcr.get_cr() as f32;

    let r = (y + 1.402*(cr - 128.0)).round() as u8;
    let g = (y - 0.344136*(cb - 128.0) - 0.714136*(cr - 128.0)).round() as u8;
    let b = (y + 1.772*(cb - 128.)).round() as u8;

    Rgba::<u8>::new(r, g, b,  255)
}

pub fn ycbcr2yuv(ycbcr: YCbCr<u8>) -> Yuv<f32> {
    let rgb = ycbcr2rgb(ycbcr);
    rgb2yuv(rgb)
}

pub fn yuv2ycbcr(yuv: Yuv<f32>) -> YCbCr<u8> {
    let rgb = yuv2rgb(yuv);
    rgb2ycbcr(rgb)
}

pub fn yuv2rgb(yuv: Yuv<f32>) -> Rgb<u8> {
    let m = Matrix3::new(
        1.0, 0.0, 1.28033,
        1.0, -0.21482, -0.38059,
        1.0, 2.12798, 0.0
    );

    let res = m*yuv.channels().map(|x| x as f64);

    Rgb::new(res[0] as u8, res[1] as u8, res[2] as u8)
}

pub fn yuv2rgba(yuv: Yuv<f32>) -> Rgba<u8> {
    let m = Matrix3::new(
        1.0, 0.0, 1.28033,
        1.0, -0.21482, -0.38059,
        1.0, 2.12798, 0.0
    );

    let res = m*yuv.channels().map(|x| x as f64);

    Rgba::new(res[0] as u8, res[1] as u8, res[2] as u8, 255)
}

pub trait ConvertPixel {
    fn to_rgb(&self) -> Rgb<u8>;
    fn to_rgba(&self) -> Rgba<u8>;
    fn to_ycbcr(&self) -> YCbCr<u8>;
    fn to_yuv(&self) -> Yuv<f32>;

    fn from_rgb(rgb: Rgb<u8>) -> Self;
    fn from_rgba(rgba: Rgba<u8>) -> Self;
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self;
    fn from_yuv(yuv: Yuv<f32>) -> Self;
}

impl ConvertPixel for Rgb<u8> {
    fn to_rgb(&self) -> Rgb<u8> { self.clone() }
    fn to_rgba(&self) -> Rgba<u8> { rgb2rgba(self.clone()) }
    fn to_ycbcr(&self) -> YCbCr<u8> { rgb2ycbcr(self.clone()) }
    fn to_yuv(&self) -> Yuv<f32> { rgb2yuv(self.clone()) }

    fn from_rgb(rgb: Rgb<u8>) -> Self { rgb }
    fn from_rgba(rgba: Rgba<u8>) -> Self { rgba2rgb(rgba) }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self { ycbcr2rgb(ycbcr) }
    fn from_yuv(yuv: Yuv<f32>) -> Self { yuv2rgb(yuv) }
}

impl ConvertPixel for Rgba<u8> {
    fn to_rgb(&self) -> Rgb<u8> { rgba2rgb(self.clone()) }
    fn to_rgba(&self) -> Rgba<u8> { self.clone() }
    fn to_ycbcr(&self) -> YCbCr<u8> { rgba2ycbcr(self.clone()) }
    fn to_yuv(&self) -> Yuv<f32> { rgba2yuv(self.clone()) }

    fn from_rgb(rgb: Rgb<u8>) -> Self { rgb2rgba(rgb) }
    fn from_rgba(rgba: Rgba<u8>) -> Self { rgba }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self { ycbcr2rgba(ycbcr) }
    fn from_yuv(yuv: Yuv<f32>) -> Self { yuv2rgba(yuv) }
}

impl ConvertPixel for YCbCr<u8> {
    fn to_rgb(&self) -> Rgb<u8> { ycbcr2rgb(self.clone()) }
    fn to_rgba(&self) -> Rgba<u8> { ycbcr2rgba(self.clone()) }
    fn to_ycbcr(&self) -> YCbCr<u8> { self.clone() }
    fn to_yuv(&self) -> Yuv<f32> { ycbcr2yuv(self.clone()) }

    fn from_rgb(rgb: Rgb<u8>) -> Self { rgb2ycbcr(rgb) }
    fn from_rgba(rgba: Rgba<u8>) -> Self { rgba2ycbcr(rgba) }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self { ycbcr }
    fn from_yuv(yuv: Yuv<f32>) -> Self { yuv2ycbcr(yuv) }
}

impl ConvertPixel for Yuv<f32> {
    fn to_rgb(&self) -> Rgb<u8> { yuv2rgb(self.clone()) }
    fn to_rgba(&self) -> Rgba<u8> { yuv2rgba(self.clone()) }
    fn to_ycbcr(&self) -> YCbCr<u8> { yuv2ycbcr(self.clone()) }
    fn to_yuv(&self) -> Yuv<f32> { self.clone() }

    fn from_rgb(rgb: Rgb<u8>) -> Self { rgb2yuv(rgb) }
    fn from_rgba(rgba: Rgba<u8>) -> Self { rgba2yuv(rgba) }
    fn from_ycbcr(ycbcr: YCbCr<u8>) -> Self { ycbcr2yuv(ycbcr) }
    fn from_yuv(yuv: Yuv<f32>) -> Self { yuv }
}

pub trait ConvertImage {
    fn to_rgb(&self) -> Image<Rgb<u8>>;
    fn to_rgba(&self) -> Image<Rgba<u8>>;
    fn to_ycbcr(&self) -> Image<YCbCr<u8>>;
    fn to_yuv(&self) -> Image<Yuv<f32>>;
}

impl<P: PixelTrait + ConvertPixel> ConvertImage for Image<P> {
    fn to_rgb(&self) -> Image<Rgb<u8>> {
        let mut new_image = Image::<Rgb<u8>>::new(self.get_width(), self.get_height());

        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                let pixel = self.get_pixel(x, y);
                let new_pixel = pixel.to_rgb();
                new_image.set_pixel(x, y, new_pixel.channels().as_slice());
            }
        }

        new_image
    }

    fn to_rgba(&self) -> Image<Rgba<u8>> {
        let mut new_image = Image::<Rgba<u8>>::new(self.get_width(), self.get_height());

        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                let pixel = self.get_pixel(x, y);
                let new_pixel = pixel.to_rgba();
                new_image.set_pixel(x, y, new_pixel.channels().as_slice());
            }
        }

        new_image
    }

    fn to_ycbcr(&self) -> Image<YCbCr<u8>> {
        let mut new_image = Image::<YCbCr<u8>>::new(self.get_width(), self.get_height());

        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                let pixel = self.get_pixel(x, y);
                let new_pixel = pixel.to_ycbcr();
                new_image.set_pixel(x, y, new_pixel.channels().as_slice());
            }
        }

        new_image
    }

    fn to_yuv(&self) -> Image<Yuv<f32>> {
        let mut new_image = Image::<Yuv<f32>>::new(self.get_width(), self.get_height());

        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                let pixel = self.get_pixel(x, y);
                let new_pixel = pixel.to_yuv();
                new_image.set_pixel(x, y, new_pixel.channels().as_slice());
            }
        }

        new_image
    }
}