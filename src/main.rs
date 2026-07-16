mod load;
mod dicompress_image;
mod flatten_img;

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


    let _img_flattened = flatten_img::flatten_img(&decompressed_img);
    
    let duration = start.elapsed();
    println!("Time: {} ms", duration.as_millis());
    
    drop(image_data);
    Ok(())
}