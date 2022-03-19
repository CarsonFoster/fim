pub trait Layout {
    fn from_qwerty(&self, qwerty_press: u8) -> u8;
    fn to_qwerty(&self, layout_press: u8) -> u8;
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
