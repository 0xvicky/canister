// use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug)]
struct CreateFile {
    file_path: String,
    file_name: String,
    content: String,
}

impl CreateFile {
    fn build(
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

// fn trim_path(input: &mut str) -> Result<String, &'static str> {}
fn handle_choice() -> Result<u32, &'static str> {
    let mut input_choice: String = String::new();
    match io::stdin().read_line(&mut input_choice) {
        Ok(_) => match input_choice.trim().parse::<u32>() {
            Ok(res) => Ok(res),
            Err(_) => Err("Error Parsing"),
        },
        Err(_) => Err("Error reading input"),
    }
    // let input_choice = input_choice.trim().parse::<u32>().expect("Cannot parse");

    // Ok(input_choice)
}

fn handle_input() -> Result<String, &'static str> {
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
            Ok(input)
        }
        Err(_) => Err("Error reading input"),
    }
}

fn create_file(config: CreateFile) -> Result<String, &'static str> {
    match fs::write(&config.file_path, config.content) {
        Ok(_) => {
            println!("File Created Successfully");
            Ok(config.file_path)
        }
        Err(_) => Err("Error writing"),
    }

    // Ok(file_path)
}

fn main() {
    loop {
        println!("What you want to do?");
        //in creation file, ask to add the content by taking the input from user and then add the content into that created file in the filesystem
        println!("1. Create File");
        println!("2. Read File Content");
        println!("3. Update File");
        println!("4. Delete File");
        println!("5. Quit");

        if let Ok(input_choice) = handle_choice() {
            match input_choice {
                1 => {
                    println!("Creating File...");
                    println!("=========================");
                    println!("Enter Directory path: ");
                    let mut directory_path = match handle_input() {
                        Ok(path) => path,
                        Err(err) => {
                            println!("Error creating directory,{}", err);
                            continue;
                        }
                    };
                    println!("Enter File name: ");
                    let mut file_name = match handle_input() {
                        Ok(file) => file,
                        Err(err) => {
                            println!("Error creating file,{}", err);
                            continue;
                        }
                    };
                    println!("Enter Content: ");
                    let mut content = match handle_input() {
                        Ok(content) => content,

                        Err(err) => {
                            println!("Error creating content,{}", err);
                            continue;
                        }
                    };

                    let config = match CreateFile::build(
                        &mut directory_path,
                        &mut file_name,
                        &mut content,
                    ) {
                        Ok(config) => config,
                        Err(err) => {
                            println!("Error creating config,{}", err);
                            continue;
                        }
                    };
                    // dbg!(config);
                    let path = match create_file(config) {
                        Ok(path) => path,
                        Err(err) => {
                            println!("Error creating config,{}", err);
                            continue;
                        }
                    };

                    println!("{}", path);
                }
                2 => {
                    println!("Reading file...");
                }
                3 => {
                    println!("Updating file...");
                }
                4 => {
                    println!("Deleting file...");
                }
                5 => {
                    println!("Bye");
                    break;
                }

                _ => {
                    println!("Invalid Choice");
                }
            }
        }
    }
}
