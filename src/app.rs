use crate::markdown;
use crossterm::event::{KeyCode, KeyEvent};
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    FileInput,
    FileSelect,
}

pub struct App {
    pub todos: Vec<String>,
    pub file_path: Option<PathBuf>,
    pub mode: AppMode,
    pub input: String,
    pub message: Option<String>,
    pub files: Vec<PathBuf>,
    pub selected_file_index: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            todos: Vec::new(),
            file_path: None,
            mode: AppMode::Normal,
            input: String::new(),
            message: Some("Press 'o' to open file, 'n' to create new file".to_string()),
            files: Vec::new(),
            selected_file_index: 0,
        }
    }
}

impl App {
    pub fn handle_input(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::Normal => self.handle_normal_mode(key),
            AppMode::FileInput => self.handle_file_input_mode(key),
            AppMode::FileSelect => self.handle_file_select_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('o') => {
                self.mode = AppMode::FileSelect;
                self.refresh_files();
                self.message =
                    Some("Use arrows to select, Enter to open, Esc to cancel".to_string());
            }
            KeyCode::Char('n') => {
                self.mode = AppMode::FileInput;
                self.input.clear();
                self.message =
                    Some("Enter new file name (Enter to confirm, Esc to cancel):".to_string());
            }
            _ => {}
        }
    }

    fn handle_file_input_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    self.create_new_file();
                }
            }
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.message = Some("Press 'o' to open file, 'n' to create new file".to_string());
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    fn handle_file_select_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if !self.files.is_empty() {
                    self.selected_file_index = self.selected_file_index.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if !self.files.is_empty() {
                    self.selected_file_index =
                        (self.selected_file_index + 1).min(self.files.len().saturating_sub(1));
                }
            }
            KeyCode::Enter => {
                if let Some(path) = self.files.get(self.selected_file_index).cloned() {
                    self.open_file(path);
                }
            }
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.message = Some("Press 'o' to open file, 'n' to create new file".to_string());
            }
            _ => {}
        }
    }

    fn refresh_files(&mut self) {
        self.files.clear();
        self.selected_file_index = 0;

        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    self.files.push(path);
                }
            }
        }
        self.files.sort();
    }

    fn create_new_file(&mut self) {
        let file_name = if self.input.ends_with(".md") {
            self.input.clone()
        } else {
            format!("{}.md", self.input)
        };

        let path = PathBuf::from(&file_name);

        if path.exists() {
            self.message = Some("File already exists".to_string());
            return;
        }

        match fs::write(&path, "") {
            Ok(_) => {
                self.file_path = Some(path);
                self.todos.clear();
                self.mode = AppMode::Normal;
                self.message = Some("File created successfully".to_string());
            }
            Err(e) => {
                self.message = Some(format!("Error creating file: {}", e));
            }
        }
    }

    fn open_file(&mut self, path: PathBuf) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.todos = markdown::parse_markdown(&content);
                self.file_path = Some(path);
                self.mode = AppMode::Normal;
                self.message = Some("File opened successfully".to_string());
            }
            Err(e) => {
                self.message = Some(format!("Error opening file: {}", e));
            }
        }
    }

    pub fn is_input_mode(&self) -> bool {
        self.mode == AppMode::FileInput
    }
}
