use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub struct FileEntry {
    pub path: PathBuf,
    pub name: OsString,
    pub size: u64,
    pub modified: SystemTime,
}

pub struct Scanner;

impl Scanner {
    pub fn scan(&self, root: &Path) -> io::Result<Vec<FileEntry>> {
        let mut files = Vec::new();
        self.scan_dir(root, &mut files)?;
        Ok(files)
    }

    fn scan_dir(&self, dir: &Path, files: &mut Vec<FileEntry>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                self.scan_dir(&path, files)?;
            } else if file_type.is_file() {
                let metadata = entry.metadata()?;

                files.push(FileEntry {
                    name: entry.file_name(),
                    path,
                    size: metadata.len(),
                    modified: metadata.modified()?,
                });
            }
        }

        Ok(())
    }
}
