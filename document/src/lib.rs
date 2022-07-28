//! A crate to represent open files ('documents').

pub mod buffer;


#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;

    fn construct_buffer(ascii: bool) -> (String, Buffer) {
        let mut string = String::new();
        if ascii {
            for _ in 0..500 {
                for c in 0u8..127u8 {
                    string.push(c as char);
                }
            }
        } else {

        }

        let clone = string.clone();
        (string, Buffer::new(clone))
    }

    #[test]
    fn single_ascii() {
        let (string, mut buf) = construct_buffer(true);
        for i in 0..string.len() {
            assert_eq!(&string[i..i + 1], buf.get((i as u16)..((i + 1) as u16)));
        }
    }
}
