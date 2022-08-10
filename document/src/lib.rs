//! A crate to represent open files ('documents').

pub mod buffer;


#[cfg(test)]
mod tests {
    use crate::buffer::{Buffer, PushError};

    struct UnicodeTestIterator {
        perm_idx: usize,
        str_idx: usize
    }

    impl UnicodeTestIterator {
        pub const PERMS: [[usize; 4]; 24] = [
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0]
        ];
        pub const THREE: &'static str = "\u{1100}\u{1161}\u{11a8}"; // 9 bytes
        pub const TWO: &'static str = "\u{0ba8}\u{0bbf}"; // 6 bytes
        pub const ONE_LONG: &'static str = "\u{fdfd}"; // 3 bytes
        pub const ONE: &'static str = "\u{00eb}"; // 2 bytes

        pub fn new() -> Self {
            UnicodeTestIterator{ perm_idx: 0, str_idx: 0 }
        }
    }

    impl Iterator for UnicodeTestIterator {

        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            let grapheme = match Self::PERMS[self.perm_idx][self.str_idx] {
                0 => Self::ONE,
                1 => Self::ONE_LONG,
                2 => Self::TWO,
                3 => Self::THREE,
                _ => unreachable!()
            };
            self.str_idx += 1;
            if self.str_idx == 4 {
                self.str_idx = 0;
                self.perm_idx += 1;
                if self.perm_idx == Self::PERMS.len() {
                    self.perm_idx = 0;
                }
            }
            Some(grapheme)
        }
    }

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
        const ASCII: &str = "tes!ting seq%uence 12&34"; // 24 bytes
        const LENGTH: usize = UnicodeTestIterator::THREE.len() + UnicodeTestIterator::TWO.len() + UnicodeTestIterator::ONE.len() + UnicodeTestIterator::ONE_LONG.len();
        const HALF_PERM: usize = (u16::MAX as usize - (ASCII.len() * 2)) / LENGTH / 2;

        let mut iter = UnicodeTestIterator::new();
        let mut graphemes = vec![];
        let mut string = String::new();

        for _ in 0..2 {
            for i in 0..ASCII.len() {
                let slice = &ASCII[i..i + 1];
                graphemes.push(slice);
                string.push_str(slice);
            }

            for _ in 0..HALF_PERM {
                let grapheme = iter.next().unwrap();
                graphemes.push(grapheme);
                string.push_str(grapheme);
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

    #[test]
    fn push_fails_immutable() {
        let mut buf = Buffer::new(String::from("hello there"));
        buf.set_immutable();
        let result = buf.push("t");
        assert_eq!(Err(PushError::ImmutableBuffer), result);
    }

    #[test]
    fn push_fails_space() {
        let mut buf = Buffer::new("a".repeat(u16::MAX as usize));
        let result = buf.push("t");
        assert_eq!(Err(PushError::NotEnoughSpace), result);

        let mut buf = Buffer::new("a".repeat(u16::MAX as usize - 1));
        let result = buf.push("t");
        assert_eq!(Ok(()), result);
        let result = buf.push("o");
        assert_eq!(Err(PushError::NotEnoughSpace), result);

        let mut buf = Buffer::new("a".repeat(u16::MAX as usize - 1));
        let result = buf.push("\u{00eb}");
        assert_eq!(Err(PushError::NotEnoughSpace), result);
    }

    #[test]
    fn push_unicode() {
        let mut buf = Buffer::new(String::new());
        let mut iter = UnicodeTestIterator::new();
        let mut graphemes = Vec::new();
        let mut sum = 0;

        for k in 0..10000 {
            let num_graphemes = buf.graphemes() as usize;
            assert_eq!(num_graphemes, graphemes.len());
            for i in 0..num_graphemes {
                let mut string = String::new();
                for j in i..=num_graphemes {
                    assert_eq!(&string[..], buf.get((i as u16)..(j as u16)));
                    if j < num_graphemes { string.push_str(graphemes[j]); }
                }
                sum += num_graphemes - i;
            }
            let grapheme = iter.next().unwrap();
            graphemes.push(grapheme);
            buf.push(grapheme).expect("Grapheme push shouldn't fail");
            if k % 10 == 0 {
                println!("{k}/10000: {sum} tests");
            }
        }
    }
}
