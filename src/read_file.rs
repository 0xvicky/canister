use crate::utils;
use std::fs;

pub fn read(file_path: &str) -> Result<String, &'static str> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => {
            println!("{}", err);
            Err("Read File Error")
        }
    }
}

pub fn read_file_action() {
    println!("Enter the file path:");
    //takes the path of the file
    let file_path = match utils::handle_input() {
        Ok(path) => {
            println!("{}", path);
            path
        }
        Err(err) => {
            println!("Error reading file,{}", err);
            return;
        }
    };
    //return the content as string as output
    match read(&file_path) {
        Ok(content) => {
            println!("{}", content);
            content
        }
        Err(err) => {
            println!("Error reading file,{}", err);
            return;
        }
    };
}
