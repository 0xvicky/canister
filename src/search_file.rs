use crate::utils::handle_input;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

pub fn search_file_in_fs() -> Result<(), Box<dyn Error>> {
    println!("Enter the file name that needs to be searched: ");
    let file_name = handle_input()?;

    println!("Searching...");
    let start_time = Instant::now();
    let paths: Vec<PathBuf> = WalkDir::new("/")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            if let Some(curr_path_os_str) = entry.path().file_name() {
                if let Some(curr_path_str) = curr_path_os_str.to_str() {
                    if curr_path_str.contains(&file_name) {
                        Some(entry.path().to_path_buf())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    paths.par_iter().for_each(|path| {
        if let Some(canonical_path) = get_canonical_path(path) {
            println!("{}", canonical_path);
        } else {
            eprintln!("Failed to get canonical path: {:?}", path);
        }
    });

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    Ok(())
}

fn get_canonical_path(path: &PathBuf) -> Option<String> {
    if let Ok(canonical_path) = path.canonicalize() {
        if let Some(canonical_str) = canonical_path.to_str() {
            return Some(canonical_str.replace("\\", "/"));
        }
    }
    None
}
