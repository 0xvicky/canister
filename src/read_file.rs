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
