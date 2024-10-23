use crate::app::{App, AppMode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let size = frame.size();

    match app.mode {
        AppMode::FileSelect => draw_file_select(frame, app, size),
        _ => draw_main_view(frame, app, size),
    }
}

fn draw_main_view<B: Backend>(frame: &mut Frame<B>, app: &App, size: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(size);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| ListItem::new(todo.clone()))
        .collect();

    let title = format!(
        "Todo List - {}",
        app.file_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("No file opened")
    );

    let todo_list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(todo_list, chunks[0]);

    if app.is_input_mode() {
        let input = Paragraph::new(app.input.as_ref())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, chunks[1]);
    }

    if let Some(msg) = &app.message {
        let message = Paragraph::new(msg.as_ref()).style(Style::default().fg(Color::Green));
        frame.render_widget(message, chunks[2]);
    }
}

fn draw_file_select<B: Backend>(frame: &mut Frame<B>, app: &App, size: Rect) {
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let style = if i == app.selected_file_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(path.to_string_lossy().into_owned()).style(style)
        })
        .collect();

    let file_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select File (Enter to open, Esc to cancel)"),
    );

    frame.render_widget(file_list, size);
}
