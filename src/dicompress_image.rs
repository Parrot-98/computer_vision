use image::{RgbImage};
use std::io::Cursor;


pub fn uncompress(img: &[u8]) -> RgbImage {
    let reader = image::ImageReader::new(Cursor::new(img))
        .with_guessed_format()
        .expect("Failed to read image format from byte stream");

    let dynamic_image = reader.decode()
        .expect("Failed to decode the image data"); // decode image

    dynamic_image.into_rgb8() // into standerd rgb
}