#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
