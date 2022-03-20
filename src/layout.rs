use crossterm::event::KeyCode;

pub trait Layout {
    fn from_qwerty(&self, qwerty_press: u8) -> u8;
    fn to_qwerty(&self, layout_press: u8) -> u8;
    fn from_qwerty_keycode(&self, qwerty_press: KeyCode) -> KeyCode {
        match qwerty_press {
            KeyCode::Char(c) => {
                if c.len_utf8() > 1 { qwerty_press } else { KeyCode::Char( self.from_qwerty(c as u8) as char ) }
            },
            _ => qwerty_press
        }
    }
    fn to_qwerty_keycode(&self, layout_press: KeyCode) -> KeyCode {
        match layout_press {
            KeyCode::Char(c) => {
                if c.len_utf8() > 1 { layout_press } else { KeyCode::Char( self.to_qwerty(c as u8) as char ) }
            },
            _ => layout_press
        }
    }
}

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

pub struct Qwerty;

impl Layout for Qwerty {
    fn from_qwerty(&self, qwerty_press: u8) -> u8 {
        qwerty_press
    }

    fn to_qwerty(&self, layout_press: u8) -> u8 {
        layout_press
    }
}

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
