use crate::color::YCbCr;

fn cmp_convert (superpixel: Vec<YCbCr<u8>>) -> Vec<YCbCr<i16>> {
    let mut cmp_superpixel: Vec<YCbCr<i16>> = Vec::new();
    for pixel in superpixel { cmp_superpixel.push(YCbCr::from_u8(pixel)); }
    return cmp_superpixel;
}

fn horizontal_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let horizontal_y = cmp_superpixel[1].get_y() - cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y() - cmp_superpixel[0].get_y();
    let horizontal_cb = cmp_superpixel[1].get_cb() - cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb() - cmp_superpixel[0].get_cb();
    let horizontal_cr = cmp_superpixel[1].get_cr() - cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr() - cmp_superpixel[0].get_cr();
    return YCbCr::new_u8(horizontal_y as u8, horizontal_cb as u8, horizontal_cr as u8);
}

fn vertical_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let vertical_y = cmp_superpixel[2].get_y() - cmp_superpixel[0].get_y() + cmp_superpixel[3].get_y() - cmp_superpixel[1].get_y();
    let vertical_cb = cmp_superpixel[2].get_cb() - cmp_superpixel[0].get_cb() + cmp_superpixel[3].get_cb() - cmp_superpixel[1].get_cb();
    let vertical_cr = cmp_superpixel[2].get_cr() - cmp_superpixel[0].get_cr() + cmp_superpixel[3].get_cr() - cmp_superpixel[1].get_cr();
    return YCbCr::new_u8(vertical_y as u8, vertical_cb as u8, vertical_cr as u8);
}

fn diagonal_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let diagonal_y = cmp_superpixel[0].get_y() - cmp_superpixel[1].get_y() - cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y();
    let diagonal_cb = cmp_superpixel[0].get_cb() - cmp_superpixel[1].get_cb() - cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb();
    let diagonal_cr = cmp_superpixel[0].get_cr() - cmp_superpixel[1].get_cr() - cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr();
    return YCbCr::new_u8(diagonal_y as u8, diagonal_cb as u8, diagonal_cr as u8);
}

fn average(superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let average_y = (cmp_superpixel[0].get_y() + cmp_superpixel[1].get_y() + cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y()) / 4;
    let average_cb = (cmp_superpixel[0].get_cb() + cmp_superpixel[1].get_cb() + cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb()) / 4;
    let average_cr = (cmp_superpixel[0].get_cr() + cmp_superpixel[1].get_cr() + cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr()) / 4;
    return YCbCr::new_u8(average_y as u8, average_cb as u8, average_cr as u8);
}

fn reduce_sp(superpixel: &Vec<YCbCr<u8>>) -> Vec<YCbCr<u8>> {
    let horizontal = horizontal_diff(superpixel.to_vec());
    let vertical = vertical_diff(superpixel.to_vec());
    let diagonal = diagonal_diff(superpixel.to_vec());
    let average = average(superpixel.to_vec());
    
    return vec![average, horizontal, vertical, diagonal];
}

pub fn reduce_image(superpixels: Vec<Vec<YCbCr<u8>>>) -> Vec<Vec<YCbCr<u8>>> {
    let mut reduced: Vec<Vec<YCbCr<u8>>> = Vec::new();
    for superpixel in superpixels { reduced.push(reduce_sp(&superpixel)); }

    return reduced;
}