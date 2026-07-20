use image::RgbImage;
use fast_image_resize as fr;
use std::num::NonZeroU32;

pub fn flatten_img(img: &RgbImage) -> Vec<f32> {
    let width = NonZeroU32::new(img.width()).expect("Width >! 0");
    let height = NonZeroU32::new(img.height()).expect("Height >! 0");

    let src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.as_raw().clone(),
        fr::PixelType::U8x3, // RGB format
    ).expect("Failed to create source image"); // trun into fast_image library's custom data type

    let dst_width = NonZeroU32::new(224).unwrap();
    let dst_height = NonZeroU32::new(224).unwrap();
    let mut dst_image = fr::Image::new(
        dst_width,
        dst_height,
        fr::PixelType::U8x3,
    ); // create a canvas where the image will be drawen dim is 224x224

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    resizer.resize(&src_image.view(), &mut dst_image.view_mut()).unwrap(); //draws the image on the canvas
    let resized_bytes = dst_image.buffer();
    
    let mut normalized = Vec::with_capacity(224 * 224 * 3);
    for &byte in resized_bytes {
        normalized.push(byte as f32 / 255.0);
    } // flattens the data in to this range 0.0 -> 1.0

    normalized
}