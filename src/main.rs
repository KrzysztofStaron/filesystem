use std::io::{Read, Write};

mod file_header;
use file_header::FileHeader;

fn main() {
    let file = FileHeader {
        extension: 1,
        name: *b"hello.txt\0\0\0\0\0\0\0",
        length: 1024,
        start: 1,
    };

    let bytes = file.serialize();
    let mut f = std::fs::File::create("test.img").unwrap();
    f.write_all(&bytes).unwrap();
    drop(f);

    let mut f = std::fs::File::open("test.img").unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    drop(f);

    let loaded = FileHeader::deserialize(&bytes).unwrap();
    println!(
        "type: {}, name: {:?}, length: {}, start: {}",
        loaded.extension,
        std::str::from_utf8(&loaded.name).unwrap_or("<invalid utf8>").trim_end_matches('\0'),
        loaded.length,
        loaded.start
    );
}
