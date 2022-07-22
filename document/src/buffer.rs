use unicode_segmentation::GraphemeCursor;

struct AsciiRange {
    pub byte_start: u16,
    pub length: u16,
    pub grapheme_start: u16
}

struct UnicodeRange {
    pub offset: u16,
    pub graphemes: Vec<u16>
}

/// Struct that holds part of a document.
///
/// Each `Buffer` object contains, at most, 2^16 - 1 bytes (so that it can be indexed by a u16).
/// They offer efficient, grapheme-based operations. A Rust `char`
/// [represents a Unicode codepoint, which might not be a 'user-perceived character' (grapheme)](https://doc.rust-lang.org/stable/std/primitive.char.html#representation).
/// This is particularly important for terminals, since a terminal works on a strict grid of
/// graphemes (although this is complicated by the fact that not all graphemes are one terminal 'column' or 'cell' in width).
/// Extra memory is therefore needed to keep track of graphemes.
///
/// `Buffer`s are optimized for large sections of contiguous ASCII or contiguous non-ASCII Unicode graphemes.
/// With entirely ASCII text, there is a negligible memory overhead. With entirely non-ASCII
/// Unicode grapheme text, `Buffer`s take up approximately 1.7x the size of the file (i.e. a 0.7x
/// overhead), according to my (non-rigorous) tests. However, the pathological case leads to a `Buffer` with 
/// 10x overhead (so the total size would be 11x the size of the file). For the vast majority of
/// texts you'll come across, `Buffer`s will work perfectly fine with small relative overhead in
/// exchange for grapheme-based operations.
pub struct Buffer {
    #[doc(hidden)]
    buf: String,
    #[doc(hidden)]
    ascii: Vec<AsciiRange>,
    #[doc(hidden)]
    unicode: Vec<UnicodeRange> 
}

impl Buffer {
    /// Create a new `Buffer` from a `String`.
    ///
    /// No copying is done; the `String` is moved.
    pub fn new(buf: String) -> Self {
        assert!(buf.len() <= u16::MAX.into());
        let length = buf.len() as u16;
        let b = buf.as_str();
        let mut ascii_length = 0;
        let mut unicode_length = 0;

        fn offset(ascii: bool, ascii_length: u16, unicode_length: u16) -> u16 {
            if ascii {
                unicode_length + ascii_length
            } else {
                ascii_length
            }
        }

        let mut ascii = Vec::new();
        let mut unicode: Vec<UnicodeRange> = Vec::new();
        let mut cursor = GraphemeCursor::new(0, b.len(), true);
        let mut saved_ascii_idx = None;
        let mut idx = 0u16;

        loop {
            let next_start = cursor.next_boundary(b, 0).unwrap().map(|i| i as u16).unwrap_or(length);
            let grapheme = &buf[(idx as usize)..(next_start as usize)];
            if grapheme.is_ascii() && !grapheme.is_empty() {
                if let None = saved_ascii_idx {
                    saved_ascii_idx = Some(idx);
                }
            } else {
                // either non-ascii, or end
                // in either case, idx is not included as an ascii char
                if let Some(i) = saved_ascii_idx {
                    if idx >= i + 2 {
                        // at least two ascii characters
                        let grapheme_start = offset(true, ascii_length, unicode_length);
                        ascii_length += idx - i;
                        ascii.push(AsciiRange{ length: idx - i, byte_start: i, grapheme_start });
                    } else {
                        // one ascii char
                        let offset = offset(false, ascii_length, unicode_length);
                        unicode_length += 1;
                        if Some(offset) == unicode.last().map(|r| r.offset) {
                            unicode.last_mut().unwrap().graphemes.push(idx);
                        } else {
                            unicode.push(UnicodeRange{ offset, graphemes: vec![idx] });
                        }
                    }
                    saved_ascii_idx = None;
                }
                if idx == length { break; }
                let offset = offset(false, ascii_length, unicode_length);
                unicode_length += 1;
                if Some(offset) == unicode.last().map(|r| r.offset) {
                    unicode.last_mut().unwrap().graphemes.push(idx);
                } else {
                    unicode.push(UnicodeRange{ offset, graphemes: vec![idx] });
                }
            }
            idx = next_start;
        }

        Buffer{ buf, ascii, unicode }
    }
}
