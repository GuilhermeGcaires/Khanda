use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let mut file = match File::open("./replays/7656819028.dem") {
        Ok(file) => file,
        Err(_) => panic!("Failed to open the file."),
    };
    let mut header = [0; 8];
    match file.read_exact(&mut header) {
        Ok(_) => {
            let header = String::from_utf8_lossy(&header);
            println!("Header: {}", header);
        }
        Err(_) => {
            panic!("Failed to read the file.");
        }
    }

    let mut offset_bytes = [0; 4];

    match file.read_exact(&mut offset_bytes) {
        Ok(_) => {
            let mut offset = Cursor::new(offset_bytes);

            let offset_value = offset.read_u32::<LittleEndian>().unwrap();
            println!("Offset: {:?}", offset_value);
        }
        Err(_) => {
            panic!("Failed to read the offset bytes.");
        }
    }
}
