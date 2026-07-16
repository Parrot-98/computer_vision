use std::io::Read;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tar::Archive;

pub fn get_image(target_index: usize) -> Result<Vec<u8>, anyhow::Error> {
    let hf_token = "nothing here"; // privete
    let url = "https://huggingface.co/datasets/timm/imagenet-1k-wds/resolve/main/imagenet1k-train-0000.tar?download=true";

    let mut headers = HeaderMap::new();
    let auth_value = HeaderValue::from_str(&format!("Bearer {}", hf_token))?;
    headers.insert(AUTHORIZATION, auth_value);

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(headers).send()?;

    if !response.status().is_success() {
        let status = response.status();
        // Read the error message from Hugging Face's server
        let err_body = response.text().unwrap_or_else(|_| "Could not read error body".to_string());
        
        println!("\n=== HUGGING FACE ERROR DETAILS ===");
        println!("Status Code: {}", status);
        println!("Server Message: {}", err_body);
        println!("==================================\n");

        anyhow::bail!("Request failed with status {}", status);
    }

    let mut archive = Archive::new(response);
    let mut current_image_index = 0;
    
    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?.to_path_buf();// allows the path to load on to the heap
        
        if let Some(extension) = path.extension() {
            if extension == "jpg" {
                if current_image_index == target_index {
                    println!("image at index {}: {:?}", target_index, path);
                    
                    let mut img_bytes = Vec::new();
                    entry.read_to_end(&mut img_bytes)?;
                    
                    return Ok(img_bytes);
                }
                
                current_image_index += 1;
            }
        }
    }

    anyhow::bail!("Index {} was out of bounds for this shard.", target_index)
}