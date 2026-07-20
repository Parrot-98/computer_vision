mod load;
mod dicompress_image;
mod flatten_img;
mod convolution;
mod convert;
mod max_pooling;

use std::time::Instant;

fn main() -> Result<(), anyhow::Error> {
    let target_idx = 1;
    
    let image_data = load::get_image(target_idx)?;
    println!("image data length: {} bytes", image_data.len());
    // println!("data: {:?}", image_data);
    
    let start = Instant::now();
    
    let decompressed_img = dicompress_image::uncompress(&image_data);
    let pixel = decompressed_img.get_pixel(0, 0);
    println!("{:?}", pixel);


    let img_flattened = flatten_img::flatten_img(&decompressed_img);
    println!("flatten done");

    let ndarray = convert::convert(&img_flattened);

    let convolution = convolution::convolution(&ndarray.view(), 3);

    let _max_pooling = max_pooling::max_pooling(&convolution.view(), 2);
    
    let duration = start.elapsed();
    println!("Time: {:.2} ms", duration.as_secs_f64() * 1000.0);
    
    drop(image_data);
    Ok(())
}