use crate::color::YCbCr;

/*
    Convert a superpixel from YCbCr<u8> to YCbCr<i16>
    in order to perform operations on it.
*/
fn cmp_convert (superpixel: Vec<YCbCr<u8>>) -> Vec<YCbCr<i16>> {
    let mut cmp_superpixel: Vec<YCbCr<i16>> = Vec::new();
    for pixel in superpixel { cmp_superpixel.push(YCbCr::from_u8(pixel)); }
    return cmp_superpixel;
}

/*
    Compute the horizontal difference between the four pixels of a superpixel
    and return a new pixel with the result.
*/
fn horizontal_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let horizontal_y = cmp_superpixel[1].get_y() - cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y() - cmp_superpixel[0].get_y();
    let horizontal_cb = cmp_superpixel[1].get_cb() - cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb() - cmp_superpixel[0].get_cb();
    let horizontal_cr = cmp_superpixel[1].get_cr() - cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr() - cmp_superpixel[0].get_cr();
    return YCbCr::new_u8(horizontal_y as u8, horizontal_cb as u8, horizontal_cr as u8);
}

/*
    Compute the vertical difference between the four pixels of a superpixel
    and return a new pixel with the result.
*/
fn vertical_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let vertical_y = cmp_superpixel[2].get_y() - cmp_superpixel[0].get_y() + cmp_superpixel[3].get_y() - cmp_superpixel[1].get_y();
    let vertical_cb = cmp_superpixel[2].get_cb() - cmp_superpixel[0].get_cb() + cmp_superpixel[3].get_cb() - cmp_superpixel[1].get_cb();
    let vertical_cr = cmp_superpixel[2].get_cr() - cmp_superpixel[0].get_cr() + cmp_superpixel[3].get_cr() - cmp_superpixel[1].get_cr();
    return YCbCr::new_u8(vertical_y as u8, vertical_cb as u8, vertical_cr as u8);
}

/*
    Compute the diagonal difference between the four pixels of a superpixel
    and return a new pixel with the result.
*/
fn diagonal_diff (superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let diagonal_y = cmp_superpixel[0].get_y() - cmp_superpixel[1].get_y() - cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y();
    let diagonal_cb = cmp_superpixel[0].get_cb() - cmp_superpixel[1].get_cb() - cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb();
    let diagonal_cr = cmp_superpixel[0].get_cr() - cmp_superpixel[1].get_cr() - cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr();
    return YCbCr::new_u8(diagonal_y as u8, diagonal_cb as u8, diagonal_cr as u8);
}

/*
    Calculate the average value of the four pixels of a superpixel
    and return a new pixel with the result.
*/
fn average(superpixel: Vec<YCbCr<u8>>) -> YCbCr<u8> {
    let cmp_superpixel = cmp_convert(superpixel);

    let average_y = (cmp_superpixel[0].get_y() + cmp_superpixel[1].get_y() + cmp_superpixel[2].get_y() + cmp_superpixel[3].get_y()) / 4;
    let average_cb = (cmp_superpixel[0].get_cb() + cmp_superpixel[1].get_cb() + cmp_superpixel[2].get_cb() + cmp_superpixel[3].get_cb()) / 4;
    let average_cr = (cmp_superpixel[0].get_cr() + cmp_superpixel[1].get_cr() + cmp_superpixel[2].get_cr() + cmp_superpixel[3].get_cr()) / 4;
    return YCbCr::new_u8(average_y as u8, average_cb as u8, average_cr as u8);
}

/*
    Return the new superpixel with the reduced values.
    The first (top left) pixel is the average of the four pixels of the superpixel.
    The second (top right) pixel is the vertical difference between the four pixels of the superpixel.
    The third (bottom left) pixel is the horizontal difference between the four pixels of the superpixel.
    The fourth (bottom right) pixel is the diagonal difference between the four pixels of the superpixel.
*/
fn reduce_sp(superpixel: &Vec<YCbCr<u8>>) -> Vec<YCbCr<u8>> {
    let horizontal = horizontal_diff(superpixel.to_vec());
    let vertical = vertical_diff(superpixel.to_vec());
    let diagonal = diagonal_diff(superpixel.to_vec());
    let average = average(superpixel.to_vec());
    
    return vec![average, vertical, horizontal, diagonal];
}

/*
    Reduce the entire image by reducing each superpixel.
*/
pub fn reduce_image(superpixels: Vec<Vec<YCbCr<u8>>>) -> Vec<Vec<YCbCr<u8>>> {
    let mut reduced: Vec<Vec<YCbCr<u8>>> = Vec::new();
    for superpixel in superpixels { reduced.push(reduce_sp(&superpixel)); }

    return reduced;
}