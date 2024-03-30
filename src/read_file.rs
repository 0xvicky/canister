use crate::utils;
use std::error::Error;
use std::fs;

pub fn read(file_path: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    println!("{}", content);
    Ok(())
}

pub fn read_file_action() -> Result<(), Box<dyn Error>> {
    println!("Enter the file path:");
    //takes the path of the file
    let file_path = utils::handle_input()?;

    //return the content as string as output
    read(&file_path)?;

    Ok(())
}
