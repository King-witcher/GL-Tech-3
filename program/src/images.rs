use std::io;

use image::GenericImageView;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadImageError {
    #[error("std::io error")]
    IoError(io::Error),

    #[error("Unsupported image format: {0}")]
    Unsupported(String),

    #[error("Invalid image format:")]
    InvalidImage,

    #[error("Image error: {0}")]
    Other(String),
}

pub fn get_from_file(bytes: &Vec<u8>) -> Result<gltech::Image, LoadImageError> {
    let image = image::load_from_memory(bytes)?;
    let result = gltech::Image::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        result.set(
            x,
            y,
            gltech::imaging::Color::rgb(pixel[0], pixel[1], pixel[2]),
        );
    }
    Ok(result)
}

impl From<image::ImageError> for LoadImageError {
    fn from(err: image::ImageError) -> Self {
        match err {
            image::ImageError::Unsupported(e) => {
                LoadImageError::Unsupported(e.format_hint().to_string())
            }
            image::ImageError::Decoding(_) => LoadImageError::InvalidImage,
            _ => LoadImageError::Other(err.to_string()),
        }
    }
}

impl From<io::Error> for LoadImageError {
    fn from(err: io::Error) -> Self {
        LoadImageError::IoError(err)
    }
}
