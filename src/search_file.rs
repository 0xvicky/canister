use crate::utils::handle_input;
use rayon::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

pub fn sanitize_file_path(paths: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let cononical_time = Instant::now();
    for path in paths.iter_mut() {
        if let Ok(canonical_path) = path.canonicalize() {
            if let Some(canonical_rep) = canonical_path.to_str() {
                if let Some(stripped_path) = canonical_rep.replace("\\", "/").strip_prefix("//?/") {
                    *path = PathBuf::from(stripped_path);
                }
            }
        }
    }
    let elp_can = cononical_time.elapsed();
    println!("{:?}", elp_can);
    Ok(())
}

pub fn search_n_collect(file_name: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths: Vec<PathBuf> = WalkDir::new("/")
        .into_iter()
        .par_bridge()
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
    Ok(paths)
}

pub fn search_file_in_fs() -> Result<(), Box<dyn Error>> {
    println!("Enter the file name that needs to be searched: ");
    let file_name = handle_input()?;

    println!("Searching...");
    let start_time = Instant::now();

    let mut paths = search_n_collect(&file_name)?; //search in the file system for the file_name

    let elp_time_1 = start_time.elapsed();
    println!("Elp Time 1{:?}", elp_time_1);

    sanitize_file_path(&mut paths)?; //remove the extra slashes and canonicalize the path returned from WalkDir lib
    println!("==================================");
    for path in paths.iter() {
        println!("{:?}", path); //print the path
    }
    println!("==================================");

    Ok(())
}
