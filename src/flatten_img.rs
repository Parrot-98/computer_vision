use image::RgbImage;
use fast_image_resize as fr;
use std::num::NonZeroU32;

pub fn flatten_img(img: &RgbImage) -> Vec<f32> {
    let width = NonZeroU32::new(img.width()).expect("Width must be greater than 0");
    let height = NonZeroU32::new(img.height()).expect("Height must be greater than 0");

    // 1. Create an source Image container from a copy of the raw bytes
    let src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.as_raw().clone(),
        fr::PixelType::U8x3, // RGB format
    ).expect("Failed to create source image");

    // 2. Create the destination container
    let dst_width = NonZeroU32::new(224).unwrap();
    let dst_height = NonZeroU32::new(224).unwrap();
    let mut dst_image = fr::Image::new(
        dst_width,
        dst_height,
        fr::PixelType::U8x3,
    );

    // 3. Setup the resizer with the Bilinear algorithm
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    
    // 4. Perform the resize by extracting read and write views
    resizer.resize(&src_image.view(), &mut dst_image.view_mut()).unwrap();

    // 5. Access the resized raw bytes and normalize them to f32
    let resized_bytes = dst_image.buffer();
    
    let mut normalized = Vec::with_capacity(224 * 224 * 3);
    for &byte in resized_bytes {
        normalized.push(byte as f32 / 255.0);
    }

    normalized
}