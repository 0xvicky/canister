use crate::utils;
use std::fs;
#[derive(Debug)]
pub struct CreateFile {
    file_path: String,
    file_name: String,
    content: String,
}
impl CreateFile {
    pub fn build(
        directory_path: &mut str,
        file_name: &str,
        content: &str,
    ) -> Result<CreateFile, &'static str> {
        if directory_path.is_empty() {
            return Err("Invalid directory path");
        }
        if file_name.is_empty() {
            return Err("Invalid file name");
        }
        let directory_path = directory_path.trim();
        let file_name = file_name.trim();
        let content = content.trim();

        let file_path: String = format!("{}/{}.txt", directory_path, file_name);
        println!("{}", file_path);
        Ok(CreateFile {
            file_path,
            file_name: file_name.to_owned(),
            content: content.to_owned(),
        })
    }
}

pub fn create(config: CreateFile) -> Result<String, &'static str> {
    match fs::write(&config.file_path, config.content) {
        Ok(_) => {
            println!("File Created Successfully");
            // Cipher::encrypt("")
            Ok(config.file_path)
        }
        Err(_) => Err("Error writing"),
    }

    // Ok(file_path)
}

pub fn create_file_action() {
    println!("Creating File...");
    println!("=========================");
    println!("Enter Directory path: ");
    let mut directory_path = match utils::handle_input() {
        Ok(path) => path,
        Err(err) => {
            println!("Error creating directory,{}", err);
            return;
        }
    };
    println!("Enter File name: ");
    let mut file_name = match utils::handle_input() {
        Ok(file) => file,
        Err(err) => {
            println!("Error creating file,{}", err);
            return;
        }
    };
    println!("Enter Content: ");
    let mut content = match utils::handle_input() {
        Ok(content) => content,

        Err(err) => {
            println!("Error creating content,{}", err);
            return;
        }
    };

    let config = match CreateFile::build(&mut directory_path, &mut file_name, &mut content) {
        Ok(config) => config,
        Err(err) => {
            println!("Error creating config,{}", err);
            return;
        }
    };
    // dbg!(config);
    let path = match create(config) {
        Ok(path) => path,
        Err(err) => {
            println!("Error creating config,{}", err);
            return;
        }
    };

    println!("{}", path);
}
