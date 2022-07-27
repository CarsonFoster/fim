//! A crate to represent open files ('documents').

pub mod buffer;

use crate::buffer::Buffer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
