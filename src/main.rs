mod load;

fn main() -> Result<(), anyhow::Error> {
    let target_idx = 5;
    
    let image_data = load::get_image(target_idx)?;
    println!("image data length: {} bytes", image_data.len());
    println!("data: {:?}", image_data);
    drop(image_data);
    
    Ok(())
}