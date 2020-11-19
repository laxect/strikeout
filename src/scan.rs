use crate::error::{Error, Result};
use std::{collections::HashSet, fs, path::Path};
use walkdir::{DirEntry, WalkDir};

static FILE_LIST_CACHE: &str = ".strikeout_cache";

pub fn scan_new_file(dir: &Path, file_list: &mut HashSet<String>) -> Vec<DirEntry> {
    let mut new_file_list = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = check_file(entry, file_list, &mut new_file_list) {
                log::error!("File {} process failed because of {}", file_name, e);
            }
        } else {
            log::error!("Can not access file.");
        }
    }
    new_file_list
}

fn check_file(entry: DirEntry, file_list: &mut HashSet<String>, new_file_list: &mut Vec<DirEntry>) -> Result<()> {
    if !entry.file_type().is_file() {
        return Ok(());
    }
    let path = entry.path().to_str().ok_or(Error::InvalidPath)?;
    if file_list.insert(path.to_owned()) {
        new_file_list.push(entry);
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
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

pub fn clear_cache() -> Result<()> {
    let file = fs::File::open(FILE_LIST_CACHE)?;
    file.set_len(0)?;
    Ok(())
}
