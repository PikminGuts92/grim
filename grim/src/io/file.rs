use std::{error::Error, io::Read};
use std::fs::{File, metadata, read_dir};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
pub enum FileSearchDepth {
    Immediate, // Same as Limited 0
    Limited(u32),
    Unlimited
}

pub trait PathFinder {
    fn get_all_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>>;
    fn find_files_with_depth(&self, depth: FileSearchDepth) -> Result<Vec<PathBuf>, Box<dyn Error>>;
}

impl PathFinder for Path {
    fn get_all_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut files = Vec::new();
        let depth = FileSearchDepth::Unlimited;

        find_files(&mut files, self, &depth)?;
        files.sort();

        Ok(files)
    }
    
    fn find_files_with_depth(&self, depth: FileSearchDepth) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut files = Vec::new();

        find_files(&mut files, self, &depth)?;
        files.sort();

        Ok(files)
    }
}

fn find_files(files: &mut Vec<PathBuf>, dir: &Path, search_type: &FileSearchDepth) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        // TODO: Return error?
        return Ok(())
    }

    let (search_dirs, next_depth) = match search_type {
        FileSearchDepth::Immediate => (false, FileSearchDepth::Immediate),
        FileSearchDepth::Limited(depth) => match depth {
            0 => (false, FileSearchDepth::Immediate),
            _ => (true, FileSearchDepth::Limited(depth - 1)),
        },
        FileSearchDepth::Unlimited => (true, FileSearchDepth::Unlimited),
    };

    for entry in read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() && search_dirs {
            find_files(files, &entry_path, &next_depth)?;
        } else if entry_path.is_file() {
            files.push(entry_path);
        }
    }

    Ok(())
}

pub fn get_file_size<T: AsRef<Path>>(path: T) -> u64 {
    // TODO: Safely handle
    let meta = metadata(path).unwrap();
    meta.len()
}

pub fn read_to_bytes<T: AsRef<Path>>(path: T) -> Vec<u8> {
    let mut file = File::open(path).unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    data
}