use crate::color::{YCbCr, Rgb, Rgba, Yuv};

pub fn rgb2rgba(rgb: Rgb<u8>) -> Rgba<u8> {
    Rgba::<u8>::new(rgb.get_red(), rgb.get_green(), rgb.get_blue(), 255)
}

pub fn rgb2ycbcr(rgb: Rgb<u8>) -> YCbCr<u8> {
    let r = rgb.get_red() as f32;
    let g = rgb.get_green() as f32;
    let b = rgb.get_blue() as f32;

    let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
    let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
    let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

    YCbCr::new(y, cb, cr)
}

pub fn rgba2rgb(rgba: Rgba<u8>) -> Rgb<u8> {
    Rgb::<u8>::new(rgba.get_red(), rgba.get_green(), rgba.get_blue())
}

pub fn rgba2ycbcr(rgba: Rgba<u8>) -> YCbCr<u8> {
    let r = rgba.get_red() as f32;
    let g = rgba.get_green() as f32;
    let b = rgba.get_blue() as f32;

    let y = (0. + 0.299*r + 0.587*g + 0.114*b).round() as u8;
    let cb = (128.0 - 0.168736*r - 0.331264*g + 0.5*b).round() as u8;
    let cr = (128.0 + 0.5*r - 0.418688*g - 0.081312*b).round() as u8;

    YCbCr::new(y, cb, cr)
}

pub fn ycbcr2rgb(ycbcr: YCbcr<u8>) -> Rgb<u8> {
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

pub fn ycbcr2yuv(ycbcr: YCbCr<u8>) -> Yuv<u8> {

}

pub fn yuv2ycbcr(yuv: Yuv<u8>) -> YCbCr<u8> {

}

pub fn yuv2rgb(yuv: Yuv<u8>) -> Rgb<u8> {

}

pub fn yuv2rgba(yuv: Yuv<u8>) -> Rgba<u8> {

}