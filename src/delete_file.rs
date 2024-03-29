use crate::utils;
use std::fs;
pub fn delete(file_path: &str) -> Result<(), &'static str> {
    match fs::remove_file(file_path) {
        Ok(_) => {
            println!("File Deleted Successfully");
            Ok(())
        }
        Err(err) => {
            println!("Error deleting,{}", err);
            Err("Error removing file")
        }
    }
}

pub fn delete_file_action() {
    println!("Enter file path to delete file");
    let file_path = match utils::handle_input() {
        Ok(path) => path,
        Err(err) => {
            println!("Error while deleting file, {}", err);
            return;
        }
    };
    match delete(&file_path) {
        Ok(_) => (),
        Err(_) => {
            println!("Error while deleting");
            return;
        }
    }
}
