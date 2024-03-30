mod create_file;
mod delete_file;
mod read_file;
mod update_file;
mod utils;

use create_file::create_file_action;
use delete_file::delete_file_action;
use read_file::read_file_action;
use std::error::Error;
use update_file::update_file_action;

// use std::error::Error;
#[derive(PartialEq, Eq)]
enum Action {
    CreateFile,
    ReadFileContent,
    UpdateFileContent,
    DeleteFileContent,
    Quit,
    Invalid,
}

fn perform_file_action(action: &Action) -> Result<(), Box<dyn Error>> {
    match action {
        Action::CreateFile => create_file_action(),
        Action::ReadFileContent => read_file_action(),
        Action::UpdateFileContent => update_file_action(),
        Action::DeleteFileContent => delete_file_action(),
        Action::Quit => {
            println!("Quitting");
            Ok(())
        }
        Action::Invalid => {
            println!("Invalid action");
            Ok(())
        }
    }
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

        let action = match utils::handle_choice() {
            Ok(input_choice) => match input_choice {
                1 => Action::CreateFile,
                2 => Action::ReadFileContent,
                3 => Action::UpdateFileContent,
                4 => Action::DeleteFileContent,
                5 => Action::Quit,
                _ => Action::Invalid,
            },
            Err(_) => Action::Invalid,
        };
        if let Err(err) = perform_file_action(&action) {
            eprintln!("Error: {}", err);
            continue;
        }

        if action == Action::Quit {
            break;
        }
    }
}
