use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;

pub fn project_directory() -> PathBuf {
    // ProjectDirs::from("me", "zsigsza", "dotrinox-manager")
    //     .unwrap()
    //     .config_local_dir()
    //     .to_owned()
    PathBuf::from("c:/dotrinox-manager")
}

pub fn config_file_path() -> PathBuf {
    project_directory().join("config.json")
}

pub fn data_file_path() -> PathBuf {
    project_directory().join("data.json")
}

pub fn create_directory(folder_name: PathBuf) {
    if !fs::metadata(&folder_name).is_ok() {
        // If the folder doesn't exist, create it
        if let Err(err) = fs::create_dir(&folder_name) {
            println!("Error creating folder: {}", err);
        } else {
        }
    } else {
    }
}

pub fn file_exists(file_path: PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.is_file() {
            true
        } else {
            false
        }
    } else {
        false
    }
}
pub fn read_file(path: PathBuf) -> String {
    fs::read_to_string(path).unwrap()
}
