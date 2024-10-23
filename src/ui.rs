use crate::app::App;

use tui::backend::Backend;
use tui::layout::{Constraints, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| ListItem::new(todo.clone()))
        .collect();

    let todo_list =
        List::new(items).block(Block::default().borders(Borders::ALL).title("TODO_LIST"));

    frame.render_widget(todo_list, chunks[0]);
}
