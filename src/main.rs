#[warn(missing_docs)]

mod editor;

use editor::Editor;

fn main() {
    Editor::init().unwrap().run();
}
