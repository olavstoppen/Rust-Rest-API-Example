use crate::config::Config;
use chrono::prelude::*;
use image::{io::Reader, GenericImageView, ImageOutputFormat};
use std::fs;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

pub async fn save_image(config: &Config, image_bytes: Vec<u8>, new_width: u32) -> String {
    let current_year = chrono::Utc::now().year();
    let current_month = format!("{:02}", Utc::now().month());
    let folder_path = format!("images/{}/{}", current_year, current_month);
    let filename = format!("{}.jpg", uuid::Uuid::new_v4().to_string());
    if !Path::new(&folder_path).exists() {
        if let Err(_) = std::fs::create_dir_all(&folder_path) {
            return config.url_default_images.to_owned();
        }
    }
    let mut file_path = folder_path.clone();
    file_path.push('/');
    file_path.push_str(&filename);

    let image = match Reader::new(std::io::Cursor::new(image_bytes)).with_guessed_format() {
        Ok(reader) => reader.decode().unwrap(),
        Err(_) => {
            return config.url_default_images.to_owned();
        }
    };

    let (original_width, original_height) = image.dimensions();
    let new_height = (original_height as f32 * (new_width as f32 / original_width as f32)) as u32;
    let resized_image = image.resize(new_width, new_height, image::imageops::FilterType::Triangle);

    let mut f = match File::create(&file_path) {
        Ok(f) => f,
        Err(_) => {
            return config.url_default_images.to_owned();
        }
    };

    if let Err(_) = resized_image.write_to(&mut f, ImageOutputFormat::Jpeg(80)) {
        return config.url_default_images.to_owned();
    }

    // Remove multipart header
    let file_length = match f.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => {
            return config.url_default_images.to_owned();
        }
    };
    if let Err(_) = f.seek(SeekFrom::Start(file_length + 4)) {
        return config.url_default_images.to_owned();
    }
    if let Err(_) = f.sync_all() {
        return config.url_default_images.to_owned();
    }
    return format!("http://127.0.0.1:8484/{}",file_path)
    
}

pub fn delete_profile_image(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}
