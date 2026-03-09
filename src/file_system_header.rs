use crate::file_header::FileHeader;
use crate::file_header::FILE_HEADER_LENGTH;
use crate::{BLOCK_SIZE};

pub const HEADER_OFFSET: usize = 5;

#[repr(C)]
pub struct FileSystemHeader {
    pub count: u8,
    pub disc_size: u32,
    pub content: Vec<FileHeader>,
}

impl FileSystemHeader {
    pub fn calc_size(&self) -> usize {
        HEADER_OFFSET as usize + self.count as usize * FILE_HEADER_LENGTH
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.calc_size());

        bytes.push(self.count);
        bytes.extend_from_slice(&self.disc_size.to_le_bytes());

        for fh in &self.content {
            bytes.extend(fh.serialize());
        }

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < HEADER_OFFSET {
            return None;
        }

        let count = bytes[0];
        let disc_size: u32 = u32::from_le_bytes(bytes[1..5].try_into().unwrap());

        if bytes.len() < HEADER_OFFSET + (count as usize) * FILE_HEADER_LENGTH {
            return None;
        }

        let mut content = Vec::with_capacity(count as usize);

        let content_bytes_len = count as usize * FILE_HEADER_LENGTH;
        let content_bytes: &[u8] = &bytes[HEADER_OFFSET..HEADER_OFFSET + content_bytes_len];

        for i in 0..(count as usize) {
            let slice_start = i*FILE_HEADER_LENGTH;
            let slice_end = (i+1) * FILE_HEADER_LENGTH;

            let slice = &content_bytes[slice_start..slice_end];
            
            if let Some(fh) = FileHeader::deserialize(slice) {
                content.push(fh);
            }
        }

        Some(Self { count, disc_size, content })
    }

    pub fn data_start_offset(&self) -> usize {
        let blocks_needed = (self.calc_size() + BLOCK_SIZE - 1) / BLOCK_SIZE;
        blocks_needed * BLOCK_SIZE
    }
}

