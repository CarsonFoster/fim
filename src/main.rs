mod editor;
mod layout;

use editor::Editor;

fn main() {
    let mut fim = Editor::new();
    if let Err(e) = fim.run() {
        std::mem::drop(fim);
        println!("[-] Application error: {}", e);
        std::process::exit(1);
    }
}
