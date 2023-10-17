#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

mod terminal;
pub use terminal::Terminal;

fn main() {
    Editor::new().run();
}
