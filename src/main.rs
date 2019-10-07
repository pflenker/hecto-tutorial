#![warn(clippy::all, clippy::pedantic)]
mod editor;

use editor::Editor;

fn main() {
    Editor::default().run();
}
