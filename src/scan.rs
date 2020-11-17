use std::{collections::HashSet, ffi};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Error, Debug)]
enum ScanError {
    #[error("Not a valid path.")]
    InvalidPath,
}

type Result<T> = std::result::Result<T, ScanError>;

pub fn scan_new_file(dir: &str, file_list: &mut HashSet<String>) -> Vec<DirEntry> {
    let mut new_file_list = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = check_file(entry, file_list, &mut new_file_list) {
                println!("File {} process failed because of {}", file_name, e);
            }
        } else {
            println!("Can not access file.");
        }
    }
    new_file_list
}

fn check_file(entry: DirEntry, file_list: &mut HashSet<String>, new_file_list: &mut Vec<DirEntry>) -> Result<()> {
    if !entry.file_type().is_file() {
        return Ok(());
    }
    let path = entry.path().to_str().ok_or(ScanError::InvalidPath)?;
    if !file_list.contains(path) {
        file_list.insert(path.to_string());
        new_file_list.push(entry);
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

pub fn get_old_file_list() -> HashSet<String> {
    HashSet::new()
}

pub fn store_old_file_list(old_file_list: &HashSet<String>) {
    // pass
}
