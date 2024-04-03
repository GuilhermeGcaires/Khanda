use crate::dota;
use std::io::{self, Read};

const BUFFER_SIZE: usize = 1024 * 100;

pub struct Stream<R: Read> {
    reader: R,
    buf: Vec<u8>,
    size: usize,
}

impl<R: Read> Stream<R> {
    pub fn new(reader: R) -> Self {
        Stream {
            reader,
            buf: vec![0; BUFFER_SIZE],
            size: BUFFER_SIZE,
        }
    }

    pub fn read_bytes(&mut self, n: usize) -> io::Result<&[u8]> {
        if n > self.size {
            self.buf.resize(n, 0);
            self.size = n;
        }

        self.reader.read_exact(&mut self.buf[..n])?;
        Ok(&self.buf[..n])
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_var_uint32(&mut self) -> io::Result<u32> {
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        loop {
            let b = self.read_byte()? as u32;
            let u = b & 0x7F;
            x |= u << y;
            y += 7;
            if (b & 0x80) == 0 || y == 35 {
                break;
            }
        }

        Ok(x)
    }

    pub fn read_command(&mut self) -> io::Result<dota::EDemoCommands> {
        let c = self.read_var_uint32()?;
        Ok(dota::EDemoCommands::from(c))
    }
}
