use crate::main::BLOCK_SIZE;
use crate::main::DISC_NAME;
use file_header::{Extension, FileHeader};


fn readFileData(&bytes: Vec<u8>, &file_header: FileHeader) -> Vec<u8> {
    return bytes[file_header.start*BLOCK_SIZE..(file_header.start+file_header.length)*BLOCK_SIZE];
}

fn writeFileFata(&mut bytes: Vec<u8>, &mut file_header: FileHeader, data: Vec<u8>) {
    file_header
}