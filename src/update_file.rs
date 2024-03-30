use crate::utils;
use std::error::Error;
use std::fs;

pub fn update(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(file_path, content)?;

    Ok(())
}

pub fn update_file_action() -> Result<(), Box<dyn Error>> {
    println!("Enter the file path");
    let file_path = utils::handle_input()?;

    println!("Enter the new content");
    let content = utils::handle_input()?;

    update(&file_path, &content)?;
    Ok(())
}
