mod create_file;
mod delete_file;
mod read_file;
mod update_file;
mod utils;

// use std::error::Error;
use create_file::CreateFile;
use utils::handle_choice;
use utils::handle_input;

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
                    let path = match create_file::create(config) {
                        Ok(path) => path,
                        Err(err) => {
                            println!("Error creating config,{}", err);
                            continue;
                        }
                    };

                    println!("{}", path);
                }
                2 => {
                    println!("Enter the file path:");
                    //takes the path of the file
                    let file_path = match handle_input() {
                        Ok(path) => {
                            println!("{}", path);
                            path
                        }
                        Err(err) => {
                            println!("Error reading file,{}", err);
                            continue;
                        }
                    };
                    //return the content as string as output
                    match read_file::read(&file_path) {
                        Ok(content) => {
                            println!("{}", content);
                            content
                        }
                        Err(err) => {
                            println!("Error reading file,{}", err);
                            continue;
                        }
                    };
                }
                3 => {
                    println!("Enter the file path");
                    let file_path = match handle_input() {
                        Ok(path) => path,
                        Err(err) => {
                            println!("Error reading file path,{}", err);
                            continue;
                        }
                    };

                    println!("Enter the new content");
                    let content = match handle_input() {
                        Ok(content) => content,
                        Err(err) => {
                            println!("Error while reading content, {}", err);
                            continue;
                        }
                    };

                    match update_file::update(&file_path, &content) {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Error while updating");
                            continue;
                        }
                    }
                }
                4 => {
                    println!("Enter file path to delete file");
                    let file_path = match handle_input() {
                        Ok(path) => path,
                        Err(err) => {
                            println!("Error while deleting file, {}", err);
                            continue;
                        }
                    };
                    match delete_file::delete(&file_path) {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Error while deleting");
                            continue;
                        }
                    }
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
