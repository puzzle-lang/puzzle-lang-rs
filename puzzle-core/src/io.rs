use std::fs::{read_dir, remove_file, File};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn create_file(path: &PathBuf) -> Result<File, Error> {
    File::create(path)
}

pub fn remove_files<P>(dir: &PathBuf, predicate: P) -> Result<(), Error>
where
    P: Fn(&PathBuf) -> bool,
{
    if !(dir.exists() && dir.is_dir()) {
        return Err(Error::new(ErrorKind::NotFound, "Directory not found"));
    }
    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            if predicate(&path) {
                remove_file(&path)?;
            }
        }
    }
    Ok(())
}

pub fn remove_files_recursive<P>(dir: &PathBuf, predicate: P) -> Result<(), Error>
where
    P: Fn(&PathBuf) -> bool,
{
    if !(dir.exists() && dir.is_dir()) {
        return Err(Error::new(ErrorKind::NotFound, "Directory not found"));
    }
    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            if predicate(&path) {
                remove_file(&path)?;
            }
        } else if path.is_dir() {
            remove_files_recursive(&path, &predicate)?;
        }
    }

    Ok(())
}
