use crate::utils;
use std::fs;

pub fn update(file_path: &str, content: &str) -> Result<(), &'static str> {
    match fs::write(file_path, content) {
        Ok(_) => {
            println!("File updated successfully");
            Ok(())
        }
        Err(_) => Err("Error occured while updating"),
    }
}

pub fn update_file_action() {
    println!("Enter the file path");
    let file_path = match utils::handle_input() {
        Ok(path) => path,
        Err(err) => {
            println!("Error reading file path,{}", err);
            return;
        }
    };

    println!("Enter the new content");
    let content = match utils::handle_input() {
        Ok(content) => content,
        Err(err) => {
            println!("Error while reading content, {}", err);
            return;
        }
    };

    match update(&file_path, &content) {
        Ok(_) => (),
        Err(_) => {
            println!("Error while updating");
            return;
        }
    }
}
