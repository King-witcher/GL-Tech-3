use std::{fs::File, io, path::Path};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadImageError {
    #[error("std::io error")]
    IoError(io::Error),

    #[error("bmp error")]
    BmpError(bmp::BmpError),
}

pub fn load_bmp(path: &Path) -> Result<gltech::imaging::Image, LoadImageError> {
    let mut file = File::open(path)?;
    let bmp = bmp::from_reader(&mut file)?;
    let image = gltech::imaging::Image::new(bmp.get_width(), bmp.get_height());
    for (x, y) in bmp.coordinates() {
        let pixel = bmp.get_pixel(x, y);
        image.set(x, y, gltech::imaging::Color::rgb(pixel.r, pixel.g, pixel.b));
    }
    Ok(image)
}

pub fn get_image(bytes: &Vec<u8>) -> Result<gltech::imaging::Image, LoadImageError> {
    let bmp = bmp::from_reader(&mut &bytes[..])?;
    let image = gltech::imaging::Image::new(bmp.get_width(), bmp.get_height());
    for (x, y) in bmp.coordinates() {
        let pixel = bmp.get_pixel(x, y);
        image.set(x, y, gltech::imaging::Color::rgb(pixel.r, pixel.g, pixel.b));
    }
    Ok(image)
}

impl From<io::Error> for LoadImageError {
    fn from(err: io::Error) -> Self {
        LoadImageError::IoError(err)
    }
}

impl From<bmp::BmpError> for LoadImageError {
    fn from(err: bmp::BmpError) -> Self {
        LoadImageError::BmpError(err)
    }
}
