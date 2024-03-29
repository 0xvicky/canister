mod create_file;
mod delete_file;
mod read_file;
mod update_file;
mod utils;

use create_file::create_file_action;
use delete_file::delete_file_action;
use read_file::read_file_action;
use update_file::update_file_action;

// use std::error::Error;

fn main() {
    loop {
        println!("What you want to do?");
        //in creation file, ask to add the content by taking the input from user and then add the content into that created file in the filesystem
        println!("1. Create File");
        println!("2. Read File Content");
        println!("3. Update File");
        println!("4. Delete File");
        println!("5. Quit");

        if let Ok(input_choice) = utils::handle_choice() {
            match input_choice {
                1 => create_file_action(),
                2 => read_file_action(),
                3 => update_file_action(),
                4 => delete_file_action(),
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
