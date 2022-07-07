use crate::config::options::{Options, TabType};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Struct that offers grapheme-based string operations. `GraphemeString` owns its data, as its
/// name implies.
pub struct GraphemeString {
    #[doc(hidden)]
    graphemes: Vec<String>,
    #[doc(hidden)]
    cache: String,
    #[doc(hidden)]
    dirty: bool,
}

impl GraphemeString {
    /// Create a new `GraphemeString`, copying data.
    pub fn new<S: AsRef<str>>(string: S) -> Self {
        GraphemeString {
            graphemes: string.as_ref().graphemes(true).map(|slice| slice.to_owned()).collect::<Vec<String>>(),
            cache: String::new(),
            dirty: true
        }
    }

    /// Returns the `GraphemeString` as a `&str`.
    ///
    /// This updates the internal cached `String` if the `GraphemeString` has been modified since
    /// the last call to `cached`. This can be an `O(n)` operation.
    ///
    /// If `opt` is `None`, tabs are not included. Otherwise, they are rendered as spaces,
    /// according to the options in `opt`. 
    /// Note that if the string hasn't been modified, the cached
    /// `String` will be returned (and its tabs will be according to the `opt` passed to the last
    /// `cached` call that updated the internal cached `String`).
    ///
    /// # Examples
    /// ```
    /// #use crate::config::options::Options;
    /// let gs = GraphemeString::new("\t\tlet x = 1;\n");
    /// assert_eq!(gs.cached(None), "let x = 1;\n");
    /// // does not recompute tabs as according to Options
    /// assert_eq!(gs.cached(Some(Options::default())), "let x = 1;\n");
    pub fn cached(&mut self, opt: Option<&Options>) -> &str {
        if self.dirty {
            if let Some(opt) = opt {
                let tab = " ".repeat(i32::from(opt.tab_width) as usize);
                let spaces = " ".repeat(i32::from(opt.tab_spaces) as usize);
                self.cache = self.graphemes.iter().map(|s| {
                    if s == "\t" {
                        match opt.tab_type {
                            TabType::Tab => tab.as_str(),
                            TabType::Spaces => spaces.as_str()
                        }
                    } else { s.as_str() }
                }).collect::<String>();
            } else {
                self.cache = self.graphemes.iter().map(|s| s.as_str()).filter(|&s| s != "\t").collect::<String>();
            }
            self.dirty = false;
        }
        self.cache.as_str()
    }

    /// Returns the `GraphemeString` as an owned `String`, fit for serialization.
    /// 
    /// There is no caching performed. This is always `O(n)`.
    ///
    /// Tabs are rendered according to `opt`: if `opt.tab_type` is [`TabType::Tab`], all tabs are
    /// rendered as the literal tab character `\t`. Otherwise, tabs are rendered as
    /// `opt.tab_spaces` number of spaces.
    pub fn serialize(&mut self, opt: &Options) -> String {
        let spaces = " ".repeat(i32::from(opt.tab_spaces) as usize);
        self.graphemes.iter().map(|s| {
            if s == "\t" {
                match opt.tab_type {
                    TabType::Tab => s.as_str(),
                    TabType::Spaces => spaces.as_str()
                }
            } else { s.as_str() }
        }).collect::<String>()
    }

    /// Returns the width, in terminal columns, of this string.
    ///
    /// If `opt` is `None`, tabs are counted as 0 width. Otherwise, tabs are treated according to
    /// `opt`.
    pub fn width(&self, opt: Option<&Options>) -> usize {
        if let Some(opt) = opt {
            self.graphemes.iter().map(|string| {
                if string == "\t" {
                    // neither of these overflow, their values are verified
                    match opt.tab_type {
                        TabType::Tab => i32::from(opt.tab_width) as usize,
                        TabType::Spaces => i32::from(opt.tab_spaces) as usize
                    }
                } else {
                    UnicodeWidthStr::width(string.as_str())
                }
            }).sum()
        } else {
            self.graphemes.iter().map(|string| UnicodeWidthStr::width(string.as_str())).sum()
        }
    }

    /// Returns the width, in terminal columns, of the grapheme at index `idx`.
    ///
    /// If `opt` is `None`, tabs are counted as 0 width. Otherwise, tabs are treated according to
    /// `opt`.
    /// 
    /// If `idx` isn't a valid index, `None` is returned.
    pub fn width_grapheme(&self, idx: usize, opt: Option<&Options>) -> Option<usize> {
        self.graphemes.get(idx).map(|g| {
            if let Some(opt) = opt {
                if g == "\t" {
                    return match opt.tab_type {
                        TabType::Tab => i32::from(opt.tab_width) as usize,
                        TabType::Spaces => i32::from(opt.tab_width) as usize
                    };
                }
            }
            UnicodeWidthStr::width(g.as_str())
        })
    }
}
