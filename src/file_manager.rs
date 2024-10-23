use std::fs;
use std::io;
use std::path::Path;

pub struct FileManager {
    files: Vec<String>,
    selected: usize,
}

pub struct FileOperation;

impl FileManager {
    pub fn new() -> io::Result<Self> {
        let mut fm = FileManager {
            files: Vec::new(),
            selected: 0,
        };
        fm.refresh_files()?;
        Ok(fm)
    }

    pub fn refresh_files(&mut self) -> io::Result<()> {
        self.files.clear();
        self.selected = 0;

        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(path_str) = path.to_str() {
                    self.files.push(path_str.to_string());
                }
            }
        }
        self.files.sort();
        Ok(())
    }

    pub fn next(&mut self) {
        if !self.files.is_empty() {
            self.selected = (self.selected + 1) % self.files.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.files.is_empty() {
            self.selected = self.selected.checked_sub(1).unwrap_or(self.files.len() - 1);
        }
    }

    pub fn selected_file(&self) -> Option<String> {
        self.files.get(self.selected).cloned()
    }

    pub fn files(&self) -> &[String] {
        &self.files
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }
}

impl FileOperation {
    pub fn create_file(path: &str) -> io::Result<()> {
        if Path::new(path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "File already exists",
            ));
        }
        fs::write(path, "")?;
        Ok(())
    }
}
