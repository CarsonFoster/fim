//! A crate to represent open files ('documents').

pub mod buffer;
mod document;

pub use crate::document::Document;

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
    fn crlf_standard() {
       const BIG: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\r\n Semper quis lectus nulla at.\r\n Cum sociis natoque penatibus et magnis dis parturient montes.\r\n Aenean sed adipiscing diam donec adipiscing tristique risus.\r\n Quis imperdiet massa tincidunt nunc pulvinar sapien.\r\n Pretium fusce id velit ut tortor pretium viverra.\r\n At tellus at urna condimentum mattis.\r\n Massa tincidunt dui ut ornare lectus sit amet.\r\n Non curabitur gravida arcu ac tortor dignissim convallis.\r\n Dapibus ultrices in iaculis nunc sed augue lacus.\r\n Augue eget arcu dictum varius duis.\r\n Interdum consectetur libero id faucibus nisl.\r\n Pretium fusce id velit ut tortor.\r\n Pellentesque elit ullamcorper dignissim cras tincidunt lobortis.\r\n Eu feugiat pretium nibh ipsum consequat.\r\n A condimentum vitae sapien pellentesque.\r\n Ornare arcu dui vivamus arcu felis bibendum ut tristique.\r\n Tortor condimentum lacinia quis vel eros donec ac.\r\n Ultrices gravida dictum fusce ut.\r\n Ornare suspendisse sed nisi lacus sed viverra tellus in.\r\n Nam at lectus urna duis.\r\n Elit scelerisque mauris pellentesque pulvinar pellentesque habitant.\r\n Sed viverra tellus in hac habitasse.\r\n Quisque id diam vel quam elementum pulvinar.\r\n Nam at lectus urna duis convallis convallis.\r\n Amet nulla facilisi morbi tempus.\r\n Adipiscing elit ut aliquam purus sit amet luctus.\r\n Justo nec ultrices dui sapien eget mi.\r\n Ornare arcu odio ut sem nulla pharetra diam sit amet.\r\n Consequat interdum varius sit amet mattis.\r\n Suspendisse interdum consectetur libero id faucibus nisl.\r\n Odio euismod lacinia at quis risus.\r\n Lobortis feugiat vivamus at augue eget arcu dictum.\r\n Posuere ac ut consequat semper viverra.\r\n In egestas erat imperdiet sed.\r\n Eget egestas purus viverra accumsan in.\r\n Dolor sed viverra ipsum nunc aliquet bibendum enim facilisis gravida.\r\n Varius sit amet mattis vulputate enim nulla aliquet porttitor.\r\n Nunc congue nisi vitae suscipit tellus mauris a diam.\r\n Id eu nisl nunc mi ipsum.\r\n Aliquam eleifend mi in nulla posuere sollicitudin aliquam ultrices.\r\n At risus viverra adipiscing at.\r\n Maecenas pharetra convallis posuere morbi leo urna.\r\n Viverra ipsum nunc aliquet bibendum.\r\n Leo vel orci porta non.\r\n Proin nibh nisl condimentum id venenatis a condimentum.\r\n Varius sit amet mattis vulputate enim nulla aliquet porttitor.\r\n Diam sit amet nisl suscipit adipiscing bibendum est ultricies integer.\r\n Dis parturient montes nascetur ridiculus mus mauris vitae ultricies.\r\n Vivamus at augue eget arcu dictum varius.\r\n Egestas egestas fringilla phasellus faucibus scelerisque eleifend donec pretium vulputate.\r\n Adipiscing elit duis tristique sollicitudin nibh sit amet commodo.\r\n Risus at ultrices mi tempus.\r\n Felis imperdiet proin fermentum leo vel.\r\n Eget nunc scelerisque viverra mauris in aliquam sem fringilla ut.\r\n Magna ac placerat vestibulum lectus mauris.\r\n In pellentesque massa placerat duis ultricies.\r\n Euismod nisi porta lorem mollis aliquam ut porttitor leo a.\r\n Sagittis id consectetur purus ut.\r\n Nam aliquam sem et tortor consequat.\r\n";
       const BIG_REF: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n Semper quis lectus nulla at.\n Cum sociis natoque penatibus et magnis dis parturient montes.\n Aenean sed adipiscing diam donec adipiscing tristique risus.\n Quis imperdiet massa tincidunt nunc pulvinar sapien.\n Pretium fusce id velit ut tortor pretium viverra.\n At tellus at urna condimentum mattis.\n Massa tincidunt dui ut ornare lectus sit amet.\n Non curabitur gravida arcu ac tortor dignissim convallis.\n Dapibus ultrices in iaculis nunc sed augue lacus.\n Augue eget arcu dictum varius duis.\n Interdum consectetur libero id faucibus nisl.\n Pretium fusce id velit ut tortor.\n Pellentesque elit ullamcorper dignissim cras tincidunt lobortis.\n Eu feugiat pretium nibh ipsum consequat.\n A condimentum vitae sapien pellentesque.\n Ornare arcu dui vivamus arcu felis bibendum ut tristique.\n Tortor condimentum lacinia quis vel eros donec ac.\n Ultrices gravida dictum fusce ut.\n Ornare suspendisse sed nisi lacus sed viverra tellus in.\n Nam at lectus urna duis.\n Elit scelerisque mauris pellentesque pulvinar pellentesque habitant.\n Sed viverra tellus in hac habitasse.\n Quisque id diam vel quam elementum pulvinar.\n Nam at lectus urna duis convallis convallis.\n Amet nulla facilisi morbi tempus.\n Adipiscing elit ut aliquam purus sit amet luctus.\n Justo nec ultrices dui sapien eget mi.\n Ornare arcu odio ut sem nulla pharetra diam sit amet.\n Consequat interdum varius sit amet mattis.\n Suspendisse interdum consectetur libero id faucibus nisl.\n Odio euismod lacinia at quis risus.\n Lobortis feugiat vivamus at augue eget arcu dictum.\n Posuere ac ut consequat semper viverra.\n In egestas erat imperdiet sed.\n Eget egestas purus viverra accumsan in.\n Dolor sed viverra ipsum nunc aliquet bibendum enim facilisis gravida.\n Varius sit amet mattis vulputate enim nulla aliquet porttitor.\n Nunc congue nisi vitae suscipit tellus mauris a diam.\n Id eu nisl nunc mi ipsum.\n Aliquam eleifend mi in nulla posuere sollicitudin aliquam ultrices.\n At risus viverra adipiscing at.\n Maecenas pharetra convallis posuere morbi leo urna.\n Viverra ipsum nunc aliquet bibendum.\n Leo vel orci porta non.\n Proin nibh nisl condimentum id venenatis a condimentum.\n Varius sit amet mattis vulputate enim nulla aliquet porttitor.\n Diam sit amet nisl suscipit adipiscing bibendum est ultricies integer.\n Dis parturient montes nascetur ridiculus mus mauris vitae ultricies.\n Vivamus at augue eget arcu dictum varius.\n Egestas egestas fringilla phasellus faucibus scelerisque eleifend donec pretium vulputate.\n Adipiscing elit duis tristique sollicitudin nibh sit amet commodo.\n Risus at ultrices mi tempus.\n Felis imperdiet proin fermentum leo vel.\n Eget nunc scelerisque viverra mauris in aliquam sem fringilla ut.\n Magna ac placerat vestibulum lectus mauris.\n In pellentesque massa placerat duis ultricies.\n Euismod nisi porta lorem mollis aliquam ut porttitor leo a.\n Sagittis id consectetur purus ut.\n Nam aliquam sem et tortor consequat.\n";
       const EMPTY: &'static str = "";
       let mut mut_big = String::from(BIG);
       let mut empty = String::from(EMPTY);
       Buffer::convert_crlf(&mut mut_big);
       Buffer::convert_crlf(&mut empty);
       assert_eq!(BIG_REF, &mut_big[..]);
       assert_eq!(EMPTY, &empty[..]);
    }

    #[test]
    fn crlf_no_removes() {
        const REF1: &'static str = "The day was full of joy, \nbut there were no parking places for the elderly, wizened sorcerer to park in.\n He was quite angry, and yet not discouraged.\n";
        const REF2: &'static str = "The day was full of joy, but there were no parking places for the elderly, wizened sorcerer to park in. He was quite angry, and yet not discouraged.[];'\\,./`134567890-={}:\"|<>?~!@#$%^&*()_+`";
        let ref3 = "AbCdEfGhIjKlMnOp".repeat(4000);
        assert!(ref3.len() < u16::MAX as usize);
        let mut mut1 = String::from(REF1);
        let mut mut2 = String::from(REF2);
        let mut mut3 = ref3.clone();
        Buffer::convert_crlf(&mut mut1);
        Buffer::convert_crlf(&mut mut2);
        Buffer::convert_crlf(&mut mut3);
        assert_eq!(REF1, &mut1[..]);
        assert_eq!(REF2, &mut2[..]);
        assert_eq!(ref3, mut3);
    }

    #[test]
    fn crlf_standalone_cr() {
        const REF1: &'static str = "The day was full of joy, \rbut there were no parking places for the elderly, wizened sorcerer to park in.\r He was quite angry, and yet not discouraged.\r";
        const REF2: &'static str = "The day was full of joy, \r\nbut there were no parking places\r for the elderly, wizened\r sorcerer to park in. \r\nHe was quite angry, and yet not\r discouraged.\r\n";
        const REF_CMP2: &'static str = "The day was full of joy, \nbut there were no parking places\r for the elderly, wizened\r sorcerer to park in. \nHe was quite angry, and yet not\r discouraged.\n";
        let ref3 = "AbCdEfG\rIjKlMn\r\n".repeat(4000);
        let ref_cmp3 = "AbCdEfG\rIjKlMn\n".repeat(4000);
        let mut mut1 = String::from(REF1);
        let mut mut2 = String::from(REF2);
        let mut mut3 = ref3.clone();
        Buffer::convert_crlf(&mut mut1);
        Buffer::convert_crlf(&mut mut2);
        Buffer::convert_crlf(&mut mut3);
        assert_eq!(REF1, &mut1[..]);
        assert_eq!(REF_CMP2, &mut2[..]);
        assert_eq!(ref_cmp3, &mut3[..]);
    }

    #[test]
    fn crlf_unicode() {
        const STANDARD: &'static str = "Stróż pchnął kość w quiz gędźb vel fax myjń.\r\nVictor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.\r\nPříliš žluťoučký kůň úpěl ďábelské ódy.\r\nСъешь же ещё этих мягких французских булок, да выпей чаю.\r\n以呂波耳本部止千利奴流乎和加餘多連曽津祢那良牟有為能於久耶万計不己衣天阿佐伎喩女美之恵比毛勢須.\r\n";
        const STANDARD_REF: &'static str = "Stróż pchnął kość w quiz gędźb vel fax myjń.\nVictor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.\nPříliš žluťoučký kůň úpěl ďábelské ódy.\nСъешь же ещё этих мягких французских булок, да выпей чаю.\n以呂波耳本部止千利奴流乎和加餘多連曽津祢那良牟有為能於久耶万計不己衣天阿佐伎喩女美之恵比毛勢須.\n";
        const STANDALONE: &'static str = "Stróż pchnął kość w quiz gędźb vel fax myjń.\rVictor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.\rPříliš žluťoučký kůň úpěl ďábelské ódy.\rСъешь же ещё этих мягких французских булок, да выпей чаю.\r以呂波耳本部止千利奴流乎和加餘多連曽津祢那良牟有為能於久耶万計不己衣天阿佐伎喩女美之恵比毛勢須.\r";
        let mut standard = String::from(STANDARD);
        let mut no_change = String::from(STANDARD_REF);
        let mut standalone = String::from(STANDALONE);
        Buffer::convert_crlf(&mut standard);
        Buffer::convert_crlf(&mut no_change);
        Buffer::convert_crlf(&mut standalone);
        assert_eq!(STANDARD_REF, &standard[..], "standard");
        assert_eq!(STANDARD_REF, &no_change[..], "no_change");
        assert_eq!(STANDALONE, &standalone[..], "standalone");
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

    // takes a few days! passed in current version, so commented out
    /*
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
    */
}
