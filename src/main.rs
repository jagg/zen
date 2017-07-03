extern crate libc;
extern crate zen;

use std::io::Read;
use libc::termios;
use std::thread;

use zen::event::KeyReader;
use zen::keys::{Key, parse_key};

struct Termios {
    termios_c: termios,
}

impl Drop for Termios {
    fn drop(&mut self) {
        if let Err(e) = self.disable_raw_mode() {
            print!("{:?}", e);
        }

    }
}

impl Termios {
    fn new() -> Self {
        Termios { termios_c: unsafe { std::mem::uninitialized() } }
    }

    fn enable_raw_mode(&mut self) -> Result<(), String> {
        unsafe {
            if libc::tcgetattr(libc::STDIN_FILENO, &mut self.termios_c as *mut _) == -1 {
                return Err("Failed to get Termios attributes".to_owned());
            }
            let mut raw: libc::termios = self.termios_c;
            raw.c_lflag = raw.c_lflag & !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
            raw.c_iflag = raw.c_iflag &
                          !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
            raw.c_oflag = raw.c_oflag & !(libc::OPOST);
            raw.c_cflag = raw.c_cflag & !(libc::CS8);
            if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut raw as *mut _) == -1 {
                return Err("Failed to set Termios attributes".to_owned());
            }
            Ok(())
        }
    }

    fn disable_raw_mode(&mut self) -> Result<(), String> {
        unsafe {
            if libc::tcsetattr(libc::STDIN_FILENO,
                               libc::TCSAFLUSH,
                               &mut self.termios_c as *mut _) == -1 {
                return Err("Failed to set Termios attributes".to_owned());
            }
            Ok(())
        }
    }
}

fn main() {

    let mut term = Termios::new();
    let mut key_reader = KeyReader::stdin();

    term.enable_raw_mode().unwrap();

    let mut buf = [0u8; 1];
    let mut c = 0x00;

    //while c != 0x71 {
    while c != 'q' as u8 {
        if let Ok(1) = key_reader.read(&mut buf) {
            c = buf[0];
            print!("{:?}\r\n", parse_key(c));
        } else {
            print!("...");
            thread::sleep_ms(100);
        }
    }
}
