use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

pub fn parse_markdown(content: &str) -> Vec<String> {
    let arena = Arena::new();
    let root = parse_document(&arena, content, &ComrakOptions::default());
    extract_todos(root)
}

pub fn print_ast<'a>(node: &'a AstNode<'a>, indent: usize) {
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
