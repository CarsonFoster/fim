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
            _ => qwerty_press
        }
    }

    fn to_qwerty(&self, layout_press: u8) -> u8 {
        0
    }
}
