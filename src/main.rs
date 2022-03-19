mod layout;

use layout::{Qwerty, Layout};

fn main() {
    let layout = Qwerty;
    println!("'{}' in Qwerty layout: '{}'", 'A', layout.from_qwerty('A' as u8) as char);
}
