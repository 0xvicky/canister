use crate::create_file::create_file_action;
use crate::delete_file::delete_file_action;
use crate::read_file::read_file_action;
use crate::search_file::search_file_in_fs;
use crate::update_file::update_file_action;
use crate::utils;
use std::error::Error;

#[derive(PartialEq, Eq)]
enum Action {
    CreateFile,
    ReadFileContent,
    UpdateFileContent,
    DeleteFileContent,
    SearchFile,
    Quit,
    Invalid,
}

fn perform_file_action(action: &Action) -> Result<(), Box<dyn Error>> {
    match action {
        Action::CreateFile => create_file_action(),
        Action::ReadFileContent => read_file_action(),
        Action::UpdateFileContent => update_file_action(),
        Action::DeleteFileContent => delete_file_action(),
        Action::SearchFile => search_file_in_fs(),
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

fn handle_file_action() -> Result<Action, Box<dyn Error>> {
    let action = match utils::handle_choice() {
        Ok(input_choice) => match input_choice {
            1 => Action::CreateFile,
            2 => Action::ReadFileContent,
            3 => Action::UpdateFileContent,
            4 => Action::DeleteFileContent,
            5 => Action::SearchFile,
            0 => Action::Quit,
            _ => Action::Invalid,
        },
        Err(_) => Action::Invalid,
    };

    Ok(action)
}

pub fn init() -> Result<(), Box<dyn Error>> {
    loop {
        println!("What you want to do?");
        //in creation file, ask to add the content by taking the input from user and then add the content into that created file in the filesystem
        println!("1. Create File");
        println!("2. Read File Content");
        println!("3. Update File");
        println!("4. Delete File");
        println!("5. Search File");
        println!("0. Quit");

        let action = handle_file_action()?;

        if let Err(err) = perform_file_action(&action) {
            eprintln!("Error: {}", err);
            continue;
        }

        if action == Action::Quit {
            break;
        }
    }
    Ok(())
}
