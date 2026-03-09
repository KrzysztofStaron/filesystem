use std::io::{Read, Write};

mod file_header;
use file_header::{Extension, FileHeader};

mod file_system_header;
use file_system_header::{FileSystemHeader};

mod utils;

pub const BLOCK_SIZE: usize = 512; // in bytes
pub const DISC_SIZE_BLOCKS: usize = 2048; // blocks
pub const DISC_SIZE_BYTES: usize = BLOCK_SIZE*DISC_SIZE_BLOCKS;

fn main() {
    let test_file = FileHeader {
        extension: Extension::Text,
        name: *b"hello.txt\0\0\0\0\0\0\0",
        length: 0,
        start: 0,
    };

    let test_file_2 = FileHeader {
        extension: Extension::Binary,
        name: *b"world.bin\0\0\0\0\0\0\0",
        length: 0,
        start: 0,
    };

    let file_system_header = FileSystemHeader {
        count: 2,
        disc_size: DISC_SIZE_BYTES as u32,
        content: vec![test_file, test_file_2]
    };



    let bytes = file_system_header.serialize();
    let mut f = std::fs::File::create("test.img").unwrap();
    f.write_all(&bytes).unwrap();
    drop(f);

    let mut f = std::fs::File::open("test.img").unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    drop(f);

    let loaded = FileSystemHeader::deserialize(&bytes).unwrap();

    println!("File System Info: ");
    println!(
        "files: {}, disc_size: {}",
        loaded.count,
        utils::format_bytes(loaded.disc_size as u64)
    );
    println!("");


    for file in loaded.content {
        println!(
            "type: {:?}, name: {:?}, length: {}, start: {}",
            file.extension,
            std::str::from_utf8(&file.name).unwrap_or("<invalid utf8>").trim_end_matches('\0'),
            file.length,
            file.start
        );
    }

 
}
