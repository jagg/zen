use std::sync::mpsc;
use std::thread;
use std::io::Read;
use std::io;


pub struct KeyReader {
    recv: mpsc::Receiver<io::Result<u8>>,
}

impl KeyReader {
    pub fn stdin() -> KeyReader {
        let (send, recv) = mpsc::channel();

        thread::spawn(move || {
            loop {
                let opt_byte = io::stdin()
                    .bytes()
                    .next();
                if let Some(byte) = opt_byte {
                    send.send(byte).unwrap();
                }
            }
        } );
        
        KeyReader {recv: recv}
    }
}


impl Read for KeyReader {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;

        while count < buf.len() {
            match self.recv.try_recv() {
                Ok(Ok(byte)) => {
                    buf[count] = byte;
                    count += 1;
                }
                Ok(Err(e)) => return Err(e),
                Err(_)     => break,
            }
        }

        Ok(count)
    }
}


