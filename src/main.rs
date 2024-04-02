use std::{fs::File, io::Read};

fn main() {
    let mut file = match File::open("./replays/7656819028.dem") {
        Ok(file) => file,
        Err(_) => panic!("Failed to open the file."),
    };
    let mut buffer = [0; 12];
    match file.read_exact(&mut buffer) {
        Ok(_) => {
            let first_12_bytes = String::from_utf8_lossy(&buffer);
            println!("First 12 bytes: {}", first_12_bytes);
        }
        Err(_) => {
            panic!("Failed to read the file.");
        }
    }
}
