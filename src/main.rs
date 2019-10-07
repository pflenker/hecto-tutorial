#![warn(clippy::all, clippy::pedantic)]
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
