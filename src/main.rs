extern crate zen;

use std::io::Read;

use std::{thread, time};

use zen::event::KeyReader;
use zen::keys::{Key, parse_key};
use zen::terminal::{Terminal};

fn draw_rows(buffer: &mut String, cols: usize) {
    for _ in 1..cols {
        buffer.push_str("\x1b[2K");
        buffer.push_str("~\r\n");
    }
}

fn refresh_screen(term: &Terminal) {
    let mut buffer = "".to_owned();
    buffer.push_str("\x1b[?25l");
    buffer.push_str("\x1b[H");
    draw_rows(&mut buffer, term.cols);
    buffer.push_str("\x1b[H");
    buffer.push_str("\x1b[?25h");
    print!("{}", buffer);
}

fn main() {

    let mut term = Terminal::new();
    let mut key_reader = KeyReader::stdin();

    term.enable_raw_mode().unwrap();

    let mut buf = [0u8; 1];
    loop {
        refresh_screen(&term);        
        if let Ok(1) = key_reader.read(&mut buf) {
            match parse_key(buf[0]) {
                Key::Ctrl('q') => break,
                //Key::Char(ch)  => print!("{:?}\r\n", ch),
                _         => (),
            }
        } else {
            thread::sleep(time::Duration::from_millis(50));
        }
    }
    print!("\x1b[2J");
}
