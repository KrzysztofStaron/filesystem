use crate::main::BLOCK_SIZE;
use crate::main::DISC_NAME;

fn readFileData(start: u32, length: u32) -> Vec<u8> {
    let mut f = std::fs::File::open(DISC_NAME.unwrap());
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();

    return bytes[start*BLOCK_SIZE..(start+length)*BLOCK_SIZE];
}