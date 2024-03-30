use crate::utils;
use std::error::Error;
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

pub fn create(config: CreateFile) -> Result<(), Box<dyn Error>> {
    fs::write(&config.file_path, config.content)?;
    Ok(())
    // Ok(file_path)
}

pub fn create_file_action() -> Result<(), Box<dyn Error>> {
    println!("Creating File...");
    println!("=========================");

    // Get directory path
    println!("Enter Directory path: ");
    let mut directory_path = utils::handle_input()?;

    // Get file name
    println!("Enter File name: ");
    let mut file_name = utils::handle_input()?;

    // Get content
    println!("Enter Content: ");
    let mut content = utils::handle_input()?;

    // Build configuration
    let config = CreateFile::build(&mut directory_path, &mut file_name, &mut content)?;

    // Create file
    let path = create(config)?;

    // Print file path
    println!("{:?}", path);

    Ok(())
}
