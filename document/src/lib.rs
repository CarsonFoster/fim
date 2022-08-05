//! A crate to represent open files ('documents').

pub mod buffer;


#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;

    fn construct_buffer() -> (String, Buffer) {
        let mut string = String::new();
        for _ in 0..500 {
            for c in 0u8..127u8 {
                string.push(c as char);
            }
        }
        let clone = string.clone();
        (string, Buffer::new(clone))
    }

    #[test]
    fn single_ascii() {
        let (string, mut buf) = construct_buffer();
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

    #[test]
    fn unicode_all_slices() {
        const PERMS: [[usize; 4]; 24] = [
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0]
        ];
        const THREE: &str = "\u{1100}\u{1161}\u{11a8}"; // 9 bytes
        const TWO: &str = "\u{0ba8}\u{0bbf}"; // 6 bytes
        const ONE_LONG: &str = "\u{fdfd}"; // 3 bytes
        const ONE: &str = "\u{00eb}"; // 2 bytes
        const ASCII: &str = "tes!ting seq%uence 12&34"; // 24 bytes
        const HALF_PERM: usize = (u16::MAX as usize - (ASCII.len() * 2)) / (THREE.len() + TWO.len() + ONE_LONG.len() + ONE.len()) / 2;

        let mut graphemes = vec![];
        let mut string = String::new();

        for _ in 0..2 {
            for i in 0..ASCII.len() {
                let slice = &ASCII[i..i + 1];
                graphemes.push(slice);
                string.push_str(slice);
            }

            for i in 0..HALF_PERM {
                for j in PERMS[i % PERMS.len()] {
                    let grapheme = match j {
                        0 => ONE,
                        1 => ONE_LONG,
                        2 => TWO,
                        3 => THREE,
                        _ => unreachable!()
                    };
                    graphemes.push(grapheme);
                    string.push_str(grapheme);
                }
            }
        }

        /* ACTUAL TESTING STARTS BELOW */
        let mut buf = Buffer::new(string);
        println!("finished building string");
        let mut sum = 0;
        let length = graphemes.len();
        for i in 0..length {
            let mut string = String::new();
            for j in i..=length {
                assert_eq!(&string[..], buf.get((i as u16)..(j as u16)));
                if j < length { string.push_str(graphemes[j]); }
            }
            sum += length - i;
            if i % 100 == 0 {
                println!("i = {i}: did {sum} tests");
            }
        }
    }

    // takes a while! passed in current version, so commented out
    /*
    #[test]
    fn all_ascii_slices() {
        let (string, mut buf) = construct_buffer();
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
