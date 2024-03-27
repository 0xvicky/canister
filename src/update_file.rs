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
