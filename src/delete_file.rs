use crate::utils;
use std::error::Error;
use std::fs;

pub fn delete(file_path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(file_path)?;
    Ok(())
}

pub fn delete_file_action() -> Result<(), Box<dyn Error>> {
    println!("Enter file path to delete file");
    let file_path = utils::handle_input()?;
    delete(&file_path)?;
    Ok(())
}
