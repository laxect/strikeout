use std::{collections::HashSet, fs, io};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

static FILE_LIST_CACHE: &'static str = ".strikeout_cache";

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Not a valid path.")]
    InvalidPath,
    #[error("No cache file found.")]
    CacheNotFound(#[from] io::Error),
    #[error("Cache Parse Failed.")]
    InvalidCache(#[from] serde_json::Error),
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

pub fn get_file_list() -> Result<HashSet<String>> {
    let file = fs::File::open(FILE_LIST_CACHE)?;
    let file_list = serde_json::from_reader(file)?;
    Ok(file_list)
}

pub fn store_file_list(old_file_list: &HashSet<String>) -> Result<()> {
    let file = fs::File::with_options()
        .write(true)
        .create(true)
        .open(FILE_LIST_CACHE)?;
    serde_json::to_writer(file, old_file_list)?;
    Ok(())
    // pass
}
