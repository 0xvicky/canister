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
