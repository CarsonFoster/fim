//! A module for handling keyboard layouts.
use crossterm::event::KeyCode;

/// An interface for keyboard layouts.
pub trait Layout {
    /// Translate a QWERTY key press into a key press from this layout, by keyboard position.
    /// For example, a QWERTY 's' translates into a Dvorak 'o' because they are located at the same
    /// place on the keyboard.
    fn from_qwerty(&self, qwerty_press: u8) -> u8;

    /// Translate a key press from this layout into a QWERTY key press, by keyboard position.
    /// For example, a Dvorak 'e' translates into a QWERTY 'd' because they are located at the same
    /// place on the keyboard.
    fn to_qwerty(&self, layout_press: u8) -> u8;

    /// Translate a QWERTY [KeyCode](crossterm::event::KeyCode) into a KeyCode from this
    /// layout, by keyboard position. The default behavior is to return [`Self::from_qwerty()`]
    /// wrapped in a KeyCode on the enclosed character if it is an ASCII character, and otherwise
    /// return the argument.
    fn from_qwerty_keycode(&self, qwerty_press: KeyCode) -> KeyCode {
        match qwerty_press {
            KeyCode::Char(c) => {
                if c.len_utf8() > 1 { qwerty_press } else { KeyCode::Char( self.from_qwerty(c as u8) as char ) }
            },
            _ => qwerty_press
        }
    }
    
    /// Translate a [KeyCode](crossterm::event::KeyCode) from this layout into a QWERTY KeyCode, 
    /// by keyboard position. The default behavior is to return [`Self::to_qwerty()`]
    /// wrapped in a KeyCode on the enclosed character if it is an ASCII character, and otherwise
    /// return the argument.
    fn to_qwerty_keycode(&self, layout_press: KeyCode) -> KeyCode {
        match layout_press {
            KeyCode::Char(c) => {
                if c.len_utf8() > 1 { layout_press } else { KeyCode::Char( self.to_qwerty(c as u8) as char ) }
            },
            _ => layout_press
        }
    }
}

/// Maps an ASCII, QWERTY press into the ASCII character that would be created by pressing Shift
/// and that press at the same time. See also:
/// [`deshift_qwerty()`].
/// # Examples
/// ```
/// # use libfim::layout::shift_qwerty;
/// let a_shifted = shift_qwerty('a' as u8);
/// assert_eq!(a_shifted, 'A' as u8);
///
/// let three_shifted = shift_qwerty('3' as u8);
/// assert_eq!(three_shifted, '#' as u8);
///
/// let capital_a_shifted = shift_qwerty('A' as u8);
/// assert_eq!(capital_a_shifted, 'A' as u8);
///
/// let backtick_shifted = shift_qwerty('`' as u8);
/// assert_eq!(backtick_shifted, '~' as u8);
/// ```
pub fn shift_qwerty(qwerty_press: u8) -> u8 {
    // Uppercase Letters => Themselves
    // I'm not sure if this will be needed, but just in case.
    if qwerty_press >= 65 && qwerty_press <= 90 {
        return qwerty_press;
    // Lowercase Letters => Uppercase letters (- 32)
    } else if qwerty_press >= 97 && qwerty_press <= 122 {
        return qwerty_press - 32;
    }
    // Numbers, backtick, minus, equals, open/close square brackets, semicolon,
    // single quote, comma, period, forward slash, backslash, catch-all
    match qwerty_press {
        96 => 126,
        49 => 33,
        50 => 64,
        51 => 35,
        52 => 36,
        53 => 37,
        54 => 94,
        55 => 38,
        56 => 42,
        57 => 40,
        48 => 41,
        45 => 95,
        61 => 43,
        91 => 123,
        93 => 125,
        92 => 124,
        59 => 58,
        39 => 34,
        44 => 60,
        46 => 62,
        47 => 63,
        _ => qwerty_press
    }
}

/// Inverse of [`shift_qwerty()`].
///
/// When passed an ASCII character,
/// returns the ASCII character that would create this character when pressed in combination with
/// the Shift key.
/// # Examples
/// ```
/// # use libfim::layout::deshift_qwerty;
/// let capital_a_deshifted = deshift_qwerty('A' as u8);
/// assert_eq!(capital_a_deshifted, 'a' as u8);
///
/// let pound_deshifted = deshift_qwerty('#' as u8);
/// assert_eq!(pound_deshifted, '3' as u8);
///
/// let a_deshifted = deshift_qwerty('a' as u8);
/// assert_eq!(a_deshifted, 'a' as u8);
///
/// let tilde_deshifted = deshift_qwerty('~' as u8);
/// assert_eq!(tilde_deshifted, '`' as u8);
/// ```
pub fn deshift_qwerty(qwerty_shift_press: u8) -> u8 {
    // Uppercase Letters => Lowercase letters (+ 32)
    if qwerty_shift_press >= 65 && qwerty_shift_press <= 90 {
        return qwerty_shift_press + 32;
    }
    // Deshifted:
    // Numbers, backtick, minus, equals, open/close square brackets, semicolon,
    // single quote, comma, period, forward slash, backslash, catch-all
    match qwerty_shift_press {
        126 => 96,
        33 => 49,
        64 => 50,
        35 => 51,
        36 => 52,
        37 => 53,
        94 => 54,
        38 => 55,
        42 => 56,
        40 => 57,
        41 => 48,
        95 => 45,
        43 => 61,
        123 => 91,
        125 => 93,
        124 => 92,
        58 => 59,
        34 => 39,
        60 => 44,
        62 => 46,
        63 => 47,
        _ => qwerty_shift_press
    }

}

/// Struct that represents the QWERTY keyboard layout.
pub struct Qwerty;

impl Layout for Qwerty {
    fn from_qwerty(&self, qwerty_press: u8) -> u8 {
        qwerty_press
    }

    fn to_qwerty(&self, layout_press: u8) -> u8 {
        layout_press
    }
}

/// Struct that represents the [Dvorak keyboard
/// layout](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout).
///
/// Note that this is not 'Programmer Dvorak'.
pub struct Dvorak;

impl Layout for Dvorak {
    fn from_qwerty(&self, qwerty_press: u8) -> u8 {
        match qwerty_press {
            113 => 39,
            119 => 44,
            101 => 46,
            114 => 112,
            116 => 121,
            121 => 102,
            117 => 103,
            105 => 99,
            111 => 114,
            112 => 108,
            91 => 47,
            93 => 61,
            97 => 97,
            115 => 111,
            100 => 101,
            102 => 117,
            103 => 105,
            104 => 100,
            106 => 104,
            107 => 116,
            108 => 110,
            59 => 115,
            39 => 45,
            122 => 59,
            120 => 113,
            99 => 106,
            118 => 107,
            98 => 120,
            110 => 98,
            109 => 109,
            44 => 119,
            46 => 118,
            47 => 122,
            45 => 91,
            61 => 93,
            other => {
                // shift + qwerty letter => shift + dvorak equivalent
                // _, +, {, }, :, ", <, >, ? => shift + dvorak equivalent
                if (other >= 65 && other <= 90) || "_+{}:\"<>?".contains(other as char) {
                    shift_qwerty(self.from_qwerty(deshift_qwerty(other)))
                } else {
                    other
                }
            }
        }
    }

    fn to_qwerty(&self, layout_press: u8) -> u8 {
        match layout_press {
            39 => 113,
            44 => 119,
            46 => 101,
            112 => 114,
            121 => 116,
            102 => 121,
            103 => 117,
            99 => 105,
            114 => 111,
            108 => 112,
            47 => 91,
            61 => 93,
            97 => 97,
            111 => 115,
            101 => 100,
            117 => 102,
            105 => 103,
            100 => 104,
            104 => 106,
            116 => 107,
            110 => 108,
            115 => 59,
            45 => 39,
            59 => 122,
            113 => 120,
            106 => 99,
            107 => 118,
            120 => 98,
            98 => 110,
            109 => 109,
            119 => 44,
            118 => 46,
            122 => 47,
            91 => 45,
            93 => 61,
            other => {
                // shift + dvorak letter => shift + qwerty equivalent
                // _, +, {, }, :, ", <, >, ? => shift + qwerty equivalent
                if (other >= 65 && other <= 90) || "_+{}:\"<>?".contains(other as char) {
                    shift_qwerty(self.to_qwerty(deshift_qwerty(other)))
                } else {
                    other
                }
            }
        }
    }
}

/// Struct that represents the [Colemak keyboard layout](https://en.wikipedia.org/wiki/Colemak)
///
/// Note that the caps lock in not replaced by backspace due to technical limitations (crossterm
/// can't detect when the caps lock key is pressed).
pub struct Colemak;

impl Layout for Colemak {
    fn from_qwerty(&self, qwerty_press: u8) -> u8 {
        match qwerty_press {
            101 => 102,
            114 => 112,
            116 => 103,
            121 => 106,
            117 => 108,
            105 => 117,
            111 => 121,
            112 => 59,
            115 => 114,
            100 => 115,
            102 => 116,
            103 => 100,
            106 => 110,
            107 => 101,
            108 => 105,
            59 => 111,
            110 => 107,
            other => {
                // shift + qwerty letter => shift + colemak equivalent
                // : => O
                if (other >= 65 && other <= 90) || other == 58 {
                    shift_qwerty(self.from_qwerty(deshift_qwerty(other)))
                } else {
                    other
                }
            }
        }
    }

    fn to_qwerty(&self, layout_press: u8) -> u8 {
        match layout_press {
            102 => 101,
            112 => 114,
            103 => 116,
            106 => 121,
            108 => 117,
            117 => 105,
            121 => 111,
            59 => 112,
            114 => 115,
            115 => 100,
            116 => 102,
            100 => 103,
            110 => 106,
            101 => 107,
            105 => 108,
            111 => 59,
            107 => 110,
            other => {
                // shift + colemak letter => shift + qwerty equivalent
                // O => :
                if (other >= 65 && other <= 90) || other == 58 {
                    shift_qwerty(self.to_qwerty(deshift_qwerty(other)))
                } else {
                    other
                }
            }
        }
    }
}

/// Skeleton struct that represents custom, user-defined keyboard layouts (from a file).....
pub struct FromFile; 

impl FromFile {
    fn new(filename: &str) -> FromFile { 
        FromFile        
    }
}


