use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use crossterm::event::{self, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::ExecutableCommand;
use std::fs;
use std::io::stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

fn print_ast<'a>(node: &'a AstNode<'a>, indent: usize) {
    let indent_str = " ".repeat(indent);
    println!("{}{:?}", indent_str, node.data.borrow().value);
    for child in node.children() {
        print_ast(child, indent + 2);
    }
}

fn extract_todos<'a>(node: &'a AstNode<'a>) -> Vec<String> {
    let mut todos = Vec::new();
    for child in node.children() {
        match &child.data.borrow().value {
            NodeValue::Paragraph => {
                for grandchild in child.children() {
                    if let NodeValue::Text(text) = &grandchild.data.borrow().value {
                        let text_str = text.as_str();
                        if text_str.starts_with("[ ]") || text_str.starts_with("[x]") {
                            todos.push(text_str.to_string());
                        }
                    }
                }
            }
            NodeValue::Item(_) => {
                todos.extend(extract_todos(child));
            }
            _ => {
                todos.extend(extract_todos(child));
            }
        }
    }
    todos
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("todos.md").expect("Failed to read markdown file");

    let arena = Arena::new();
    let root = parse_document(&arena, &file_content, &ComrakOptions::default());

    println!("Markdown AST:");
    print_ast(root, 0);

    let todos = extract_todos(root);

    if todos.is_empty() {
        println!("No todo items found.");
    } else {
        println!("Extracted todos: {:?}", todos);
    }

    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let items: Vec<ListItem> = todos
                .iter()
                .map(|todo| ListItem::new(todo.clone()))
                .collect();
            let todo_list =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Todo List"));

            rect.render_widget(todo_list, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    terminal
        .backend_mut()
        .execute(crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
