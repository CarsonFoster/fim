use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};
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
    unicode: Vec<UnicodeRange>,
    #[doc(hidden)]
    cached_idx: Option<u16>
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

        Buffer{ buf, ascii, unicode, cached_idx: None }
    }
}

impl Buffer {
    pub fn get(&mut self, bounds: impl RangeBounds<u16>) -> &str {
        enum Index {
            Ascii(u16),
            Unicode(u16)
        }

        fn cmp_ascii(idx: u16, range: &AsciiRange, _range_idx: u16) -> Ordering {
            if idx < range.grapheme_start {
                Ordering::Less
            } else if idx >= range.grapheme_start + range.length {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

        fn cmp_unicode(idx: u16, range: &UnicodeRange, range_idx: u16) -> Ordering {
            let length = range.graphemes.len() as u16;
            let grapheme_start = range.offset + range_idx;
            if idx < grapheme_start {
                Ordering::Less
            } else if idx >= grapheme_start + length {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

        fn half(lo: u16, hi: u16) -> u16 {
            ((hi as u32 + lo as u32) >> 1) as u16
        }

        let index = if !self.ascii.is_empty() && self.ascii[0].byte_start == 0 {
            // ascii starts, ascii indexes are even
            |idx| if idx & 1 > 0 { Index::Unicode(idx >> 1) } else { Index::Ascii(idx >> 1) }
        } else {
            // unicode starts, unicode indexes are even
            |idx| if idx & 1 > 0 { Index::Ascii(idx >> 1) } else { Index::Unicode(idx >> 1) }
        };

        let binary_search = |needle: u16, lo: Option<u16>, mid: Option<u16>, hi: Option<u16>| {
            let mut lo = lo.unwrap_or(0u16);
            let mut hi = hi.unwrap_or_else(|| self.ascii.len() as u16 + self.unicode.len() as u16);
            let mut mid = mid.unwrap_or_else(|| half(lo, hi));
            while lo < hi {
                let ord = match index(mid) {
                    Index::Ascii(idx) => cmp_ascii(needle, &self.ascii[idx as usize], idx),
                    Index::Unicode(idx) => cmp_unicode(needle, &self.unicode[idx as usize], idx)
                };
                match ord {
                    Ordering::Less => hi = mid,
                    Ordering::Equal => {
                        return match index(mid) {
                            Index::Ascii(idx) => {
                                let range = &self.ascii[idx as usize];
                                range.byte_start + needle - range.grapheme_start
                            },
                            Index::Unicode(idx) => {
                                let range = &self.unicode[idx as usize];
                                let grapheme_start = range.offset + idx;
                                range.graphemes[(needle - grapheme_start) as usize]
                            }
                        };
                    },
                    Ordering::Greater => lo = mid + 1
                };
                mid = half(lo, hi);
            }
            panic!("binary search failed -- invariant violated in Buffer::get");
        };

        let start = binary_search(match bounds.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1u16,
            Bound::Unbounded => 0u16
        }, None, self.cached_idx, None);

        let end = binary_search(match bounds.end_bound() {
            Bound::Included(i) => *i + 1u16,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.buf.len() as u16
        }, Some(start), Some(start), None);

        &self.buf[start as usize..end as usize]
    }
}
