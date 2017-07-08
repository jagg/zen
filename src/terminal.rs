extern crate libc;

use libc::termios;
use std::mem;

pub struct Terminal {
    termios_c: termios,
    pub cols: usize,
    pub rows: usize,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Err(e) = self.disable_raw_mode() {
            print!("{:?}", e);
        }

    }
}

impl Terminal {
    pub fn new() -> Self {
        let (cols, rows) = get_window_size().unwrap();
        print!("{} x {}\r\n", cols, rows);
        Terminal { termios_c: unsafe { mem::uninitialized() }, cols: cols, rows: rows }
    }

    pub fn enable_raw_mode(&mut self) -> Result<(), String> {
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

    pub fn disable_raw_mode(&mut self) -> Result<(), String> {
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

fn get_window_size() -> Result<(usize, usize), String> {
    unsafe {
        let mut winsize: libc::winsize = mem::uninitialized();
        let exit_code = libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut winsize as *mut _);

        if exit_code == -1 || winsize.ws_col == 0 {
            Err("Failed to get window size".to_owned())
        } else {
            Ok((winsize.ws_col as usize, winsize.ws_row as usize))
        }
    }
}

