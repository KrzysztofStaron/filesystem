use crate::main::BLOCK_SIZE;

fn readFileData(start: u32, length: u32) -> Vec<u8> {
    let mut f = std::fs::File::open("test.img").unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();

    return bytes[start*BLOCK_SIZE..(start+length)*BLOCK_SIZE];
}