use std::io::{Read, Write};

mod file_header;
use file_header::Extension;

mod filesystem;
use filesystem::FileSystem;

mod file_system_header;
use file_system_header::{FileSystemHeader};

mod utils;

pub const BLOCK_SIZE: usize = 512; // in bytes
pub const DISC_SIZE_BLOCKS: usize = 2048; // blocks
pub const DISC_SIZE_BYTES: usize = BLOCK_SIZE*DISC_SIZE_BLOCKS;
pub const DISC_NAME: &str = "mydisk.img";

fn main() {
    init_empty_disk();

    let mut fs = FileSystem { disk: read_disk() };
    fs.create("hello.txt", Extension::Text).unwrap();
    fs.create("world.bin", Extension::Binary).unwrap();
    std::fs::write(DISC_NAME, &fs.disk).unwrap();

    let mut fs = FileSystem { disk: read_disk() };
    let header = fs.header().unwrap();

    println!("File System Info: ");
    println!(
        "files: {}, disc_size: {}, header_size: {}, data_start: {}",
        header.count,
        utils::format_bytes(header.disc_size as u64),
        utils::format_bytes(header.calc_size() as u64),
        header.data_start_offset()
    );
    println!("");

    for file in &header.content {
        println!(
            "type: {:?}, name: {:?}, length: {}, start: {}",
            file.extension,
            std::str::from_utf8(&file.name).unwrap_or("<invalid utf8>").trim_end_matches('\0'),
            file.length,
            file.start
        );
    }

    if let Some(mut file) = fs.open_mut("hello.txt") {
        let data = "Hello, world! ".repeat(50);
        file.write(data.as_bytes()).unwrap();
    }

    std::fs::write(DISC_NAME, &fs.disk).unwrap();

    let file = fs.open("hello.txt");
    let content = file.read();
    let len = content.iter().position(|&b| b == 0).unwrap_or(content.len());
    println!("read ({} bytes): {:?}", len, std::str::from_utf8(&content[..len]));
}

fn init_empty_disk() {
    let file_system_header = FileSystemHeader {
        count: 0,
        disc_size: DISC_SIZE_BYTES as u32,
        content: vec![],
    };

    let mut bytes = file_system_header.serialize();
    bytes.resize(DISC_SIZE_BYTES, 0);

    let mut f = std::fs::File::create(DISC_NAME).unwrap();
    f.write_all(&bytes).unwrap();
    drop(f);
}

fn read_disk() -> Vec<u8> {
    let mut f = std::fs::File::open(DISC_NAME).unwrap();
    let mut bytes = Vec::new();

    f.read_to_end(&mut bytes).unwrap();
    drop(f);

    bytes
}