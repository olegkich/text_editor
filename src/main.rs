mod editor;
mod terminal;
mod row;
mod document;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use document::Document;
pub use row::Row;

fn main() {
	Editor::default().run();
}
