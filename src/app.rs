use crate::markdown;
use crossterm::event::KeyEvent;
use std::fs;

pub struct App {
    pub todos: Vec<String>,
    pub file_path: String,
}

impl App {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_path)?;
        let todos = markdown::parse_markdown(&content);

        Ok(App {
            todos,
            file_path: file_path.to_string(),
        })
    }

    pub fn handle_input(&mut self, _key: KeyEvent) {
        // TODO: handle this later
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        // TODO: handle this aswell
        Ok(())
    }
}
