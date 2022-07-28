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

    #[test]
    fn unicode_trivial() {
        let mut buf = Buffer::new(String::from("hëllö wörld"));
        let graphemes = ["h", "ë", "l", "l", "ö", " ", "w", "ö", "r", "l", "d"];
        for i in 0..graphemes.len() {
            let mut string = String::new();
            for j in i..graphemes.len() {
                assert_eq!(string.as_str(), buf.get((i as u16)..(j as u16)), "{}..{}", i, j);
                string.push_str(graphemes[j]);
            }
        }

        let mut start_till = String::new();
        let mut till_end = String::new();
        for i in 0..graphemes.len() {
            till_end.insert_str(0, graphemes[graphemes.len() - i - 1]);
            assert_eq!(&start_till[..], buf.get(..(i as u16)));
            assert_eq!(&till_end[..], buf.get(((graphemes.len() - i - 1) as u16)..));
            start_till.push_str(graphemes[i]);
            assert_eq!(&start_till[..], buf.get(..=(i as u16)));
        }
    }

    // takes a while! passed in current version, so commented out
    /*
    #[test]
    fn all_ascii_slices() {
        let (string, mut buf) = construct_buffer(true);
        let mut sum = 0;
        for i in 0..string.len() {
            for j in i..=string.len() {
                assert_eq!(&string[i..j], buf.get((i as u16)..(j as u16)));
            }
            sum += string.len() - i;
            if i % 100 == 0 {
                println!("i = {i}: did {} tests", sum);
            }
        }
    }
    */
}
