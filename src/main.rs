mod create_file;
mod delete_file;
mod initialiser;
mod read_file;
mod update_file;
mod utils;
use initialiser::init;
// use std::error::Error;

fn main() {
    if let Err(err) = init() {
        eprintln!("Error while initializing:{}", err);
    }
}
