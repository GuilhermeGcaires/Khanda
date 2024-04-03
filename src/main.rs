use std::{
    collections::HashMap,
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

mod generated {
    pub mod demo;
}

use crate::generated::demo::*;

const DEM_IsCompressed: u32 = 0x8000;

struct Parser<R> {
    tick: u32,
    net_tick: u32,
    game_build: u32,
    class_id_size: u32,
    class_info: bool,
    entity_full_packets: i32,
    game_event_names: HashMap<i32, String>,
    is_stopping: bool,
    stream: R,
    stop_at_tick: u32,
}

#[derive(Debug)]
struct OuterMessage {
    tick: u32,
    type_id: i32,
    data: Vec<u8>,
}

impl<R: Read + Seek> Parser<R> {
    fn new(mut stream: R) -> Result<Self, io::Error> {
        let mut header = [0; 8];
        stream.read_exact(&mut header)?;

        let mut offset_bytes = [0; 4];
        stream.read_exact(&mut offset_bytes)?;

        let offset_value = Cursor::new(offset_bytes).read_u32::<LittleEndian>()?;
        stream.seek(SeekFrom::Start(u64::from(offset_value)))?;

        Ok(Parser {
            tick: 0,
            net_tick: 0,
            game_build: 0,
            class_id_size: 0,
            class_info: false,
            entity_full_packets: 0,
            game_event_names: HashMap::new(),
            is_stopping: false,
            stream,
            stop_at_tick: 0,
        })
    }

    fn read_outer_message(&mut self) -> Result<Option<OuterMessage>, io::Error> {
        let command = self.stream.read_u32::<LittleEndian>()?;

        let msg_type = EDemoCommands::DEM_IsCompressed;
        let msg_compressed = (EDemoCommands::DEM_IsCompressed) != 0;

        let tick = self.stream.read_u32::<LittleEndian>()?;
        let tick = if tick == std::u32::MAX { 0 } else { tick };

        println!("{:?}", tick);

        let size = self.stream.read_u32::<LittleEndian>()?;
        let mut buf = vec![0; size as usize];
        self.stream.read_exact(&mut buf)?;

        if msg_compressed {
            buf = decompress_data(&buf)?;
        }
        Ok(Some(OuterMessage {
            tick,
            type_id: msg_type as i32,
            data: buf,
        }))
    }
}

fn decompress_data(compressed_data: &[u8]) -> Result<Vec<u8>, io::Error> {
    let mut decoder = snap::read::FrameDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

fn main() {
    let file = match File::open("./replays/7656819028.dem") {
        Ok(file) => file,
        Err(e) => panic!("Failed to open the file: {}", e),
    };

    let mut parser = match Parser::new(file) {
        Ok(parser) => parser,
        Err(e) => panic!("Failed to create parser: {}", e),
    };

    while let Some(msg) = match parser.read_outer_message() {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Error reading message: {}", e);
            None
        }
    } {
        println!("Message: {:?}", msg);
    }
}
