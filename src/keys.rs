use std::ascii::AsciiExt;

#[derive(PartialEq, PartialOrd,Eq, Ord, Debug)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Raw(u8)
}

pub fn parse_key(byte: u8) -> Key {
    if byte >= 0x01 && byte <= 0x1A {
        Key::Ctrl((byte + b'a' - 0x01) as char)
    } else if byte >= 0x1C && byte <= 0x1F {
        Key::Ctrl((byte + b'4' - 0x1C) as char)
    } else if byte.is_ascii() {
        Key::Char(byte as char)
    } else {
        Key::Raw(byte)
    }
}






