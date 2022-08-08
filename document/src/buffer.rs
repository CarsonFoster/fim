use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

/// Enum that describes why [`Buffer::push`] couldn't push.
#[derive(Debug, PartialEq, Eq)]
pub enum PushError {
    /// The grapheme would cause the size of the buffer to exceed 2^16 - 1 bytes.
    NotEnoughSpace,
    /// [`Buffer::set_immutable`] has been called on the buffer.
    ImmutableBuffer
}

struct AsciiRange {
    pub byte_start: u16,
    pub length: u16,
    pub grapheme_start: u16
}

struct UnicodeRange {
    pub grapheme_start: u16,
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
    num_graphemes: u16,
    #[doc(hidden)]
    cached_idx: Option<u16>,
    #[doc(hidden)]
    mutable: bool,
}

impl Buffer {
    /// Create a new `Buffer` from a `String`.
    ///
    /// No copying is done; the `String` is moved.
    pub fn new(buf: String) -> Self {
        assert!(buf.len() <= u16::MAX.into());
        let length = buf.len() as u16;
        let b = buf.as_str();
        let mut num_graphemes = 0;

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
                        ascii.push(AsciiRange{ length: idx - i, byte_start: i, grapheme_start: num_graphemes });
                        num_graphemes += idx - i;
                    } else {
                        // one ascii char
                        if Some(num_graphemes) == unicode.last().map(|r| r.grapheme_start + r.graphemes.len() as u16) {
                            unicode.last_mut().unwrap().graphemes.push(i);
                        } else {
                            unicode.push(UnicodeRange{ grapheme_start: num_graphemes, graphemes: vec![i] });
                        }
                        num_graphemes += 1;
                    }
                    saved_ascii_idx = None;
                }
                if idx == length { break; }
                if Some(num_graphemes) == unicode.last().map(|r| r.grapheme_start + r.graphemes.len() as u16) {
                    unicode.last_mut().unwrap().graphemes.push(idx);
                } else {
                    unicode.push(UnicodeRange{ grapheme_start: num_graphemes, graphemes: vec![idx] });
                }
                num_graphemes += 1;
            }
            idx = next_start;
        }

        Buffer{ num_graphemes, buf, ascii, unicode, cached_idx: None, mutable: true }
    }

    /// Returns a slice of the underlying buffer.
    ///
    /// The indices are grapheme indices, not codepoint indices or byte offsets.
    ///
    /// # Panics
    /// No bounds checking is done. `get` will panic if it is given invalid indices.
    pub fn get(&mut self, bounds: impl RangeBounds<u16>) -> &str {
        if self.buf.len() == 0 { return ""; }
        enum Index {
            Ascii(u16),
            Unicode(u16)
        }

        fn cmp_ascii(idx: u16, range: &AsciiRange) -> Ordering {
            if idx < range.grapheme_start {
                Ordering::Less
            } else if idx >= range.grapheme_start + range.length {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

        fn cmp_unicode(idx: u16, range: &UnicodeRange) -> Ordering {
            let length = range.graphemes.len() as u16;
            if idx < range.grapheme_start {
                Ordering::Less
            } else if idx >= range.grapheme_start + length {
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
            if needle == self.num_graphemes {
                return (half(0, self.ascii.len() as u16 + self.unicode.len() as u16), self.buf.len() as u16);
            }
            let mut lo = lo.unwrap_or(0u16);
            let mut hi = hi.unwrap_or_else(|| self.ascii.len() as u16 + self.unicode.len() as u16);
            let mut mid = mid.unwrap_or_else(|| half(lo, hi));
            while lo < hi {
                let ord = match index(mid) {
                    Index::Ascii(idx) => cmp_ascii(needle, &self.ascii[idx as usize]),
                    Index::Unicode(idx) => cmp_unicode(needle, &self.unicode[idx as usize])
                };
                match ord {
                    Ordering::Less => hi = mid,
                    Ordering::Equal => {
                        return (mid, match index(mid) {
                            Index::Ascii(idx) => {
                                let range = &self.ascii[idx as usize];
                                range.byte_start + needle - range.grapheme_start
                            },
                            Index::Unicode(idx) => {
                                let range = &self.unicode[idx as usize];
                                range.graphemes[(needle - range.grapheme_start) as usize]
                            }
                        });
                    },
                    Ordering::Greater => lo = mid + 1
                };
                mid = half(lo, hi);
            }
            panic!("binary search failed -- invariant violated in Buffer::get");
        };

        let (start_chunk, start) = binary_search(match bounds.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1u16,
            Bound::Unbounded => 0u16
        }, None, self.cached_idx, None);

        let (end_chunk, end) = binary_search(match bounds.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.num_graphemes
        }, Some(start_chunk), Some(start_chunk), None);

        self.cached_idx = Some(end_chunk);
        &self.buf[start as usize..end as usize]
    }

    /// Attempts to push the passed grapheme to the buffer.
    ///
    /// Only attempts to push the first grapheme in `grapheme`. Returns `Ok(())` if the grapheme
    /// was successfully pushed to the buffer, and `Err(e)` otherwise, where `e` is a [`PushError`]
    /// describing the reason for failure.
    pub fn push(&mut self, grapheme: &str) -> Result<(), PushError> {
        if !self.mutable { return Err(PushError::ImmutableBuffer); }
        let grapheme = grapheme.graphemes(true).nth(0);
        if let Some(grapheme) = grapheme {
            let additional_bytes = grapheme.len();
            if additional_bytes + self.buf.len() > u16::MAX as usize {
                return Err(PushError::NotEnoughSpace);
            }
            let idx = self.buf.len() as u16;
            if grapheme.is_ascii() {
                if Some(self.num_graphemes) == self.ascii.last().map(|r| r.grapheme_start + r.length) {
                    self.ascii.last_mut().unwrap().length += 1;
                } else {
                    self.ascii.push(AsciiRange{ grapheme_start: self.num_graphemes, length: 1, byte_start: idx });
                }
            } else {
                if Some(self.num_graphemes) == self.unicode.last().map(|r| r.grapheme_start + r.graphemes.len() as u16) {
                    self.unicode.last_mut().unwrap().graphemes.push(idx);
                } else {
                    self.unicode.push(UnicodeRange{ grapheme_start: self.num_graphemes, graphemes: vec![idx] });
                }
            }
            self.num_graphemes += 1;
            self.buf.push_str(grapheme);
        }
        // if there was no grapheme, no error, since the requested grapheme, nothing, was added
        Ok(())
    }

    /// Forbid future appending to this `Buffer`.
    ///
    /// Cannot be reversed.
    pub fn set_immutable(&mut self) {
        self.mutable = false;
    }

    /// Returns the number of graphemes in this `Buffer`.
    pub fn graphemes(&self) -> u16 {
        self.num_graphemes
    }

    /// Returns the number of bytes in this `Buffer`.
    pub fn bytes(&self) -> u16 {
        self.buf.len() as u16
    }
}
