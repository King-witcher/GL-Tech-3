use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::Read,
    path::Path,
};

use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum LoadFilesError {
    #[error("Failed to load base files")]
    IoError(std::io::Error),

    #[error("Invalid base files")]
    ZipError(zip::result::ZipError),
}

pub fn load_file_system() -> Result<HashMap<String, Vec<u8>>, LoadFilesError> {
    let base_path = Path::new("base");
    let paths = fs::read_dir(base_path)?;
    let mut hash_map = HashMap::new();
    for entry in paths {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() && path.extension() == Some(OsStr::new("pk5")) {
            let files_loaded = load_pack(&mut hash_map, &path)?;
            println!("Loaded {} files from {:?}", files_loaded, path);
        }
    }

    Ok(hash_map)
}

fn load_pack(
    hash_map: &mut HashMap<String, Vec<u8>>,
    path: &Path,
) -> Result<usize, LoadFilesError> {
    let file = File::open(path)?;
    let mut count = 0;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let name = file.name().to_string();
        let bytes = file.bytes().map(Result::unwrap).collect::<Vec<u8>>();
        if !hash_map.contains_key(&name) {
            println!("load {}", name);
            count += 1;
            hash_map.insert(name, bytes);
        }
    }
    Ok(count)
}

impl From<std::io::Error> for LoadFilesError {
    fn from(err: std::io::Error) -> Self {
        LoadFilesError::IoError(err)
    }
}

impl From<zip::result::ZipError> for LoadFilesError {
    fn from(err: zip::result::ZipError) -> Self {
        LoadFilesError::ZipError(err)
    }
}
