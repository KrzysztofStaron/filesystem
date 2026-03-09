use crate::file_header::FileHeader;
use crate::file_header::FILE_HEADER_LENGTH;

const OFFSET: usize = 1;

#[repr(C)]
pub struct FileSystemHeader {
    pub count: u8,
    pub content: Vec<FileHeader>,
}

impl FileSystemHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let vec_len: usize = OFFSET as usize + self.count as usize * FILE_HEADER_LENGTH;

        let mut bytes = Vec::with_capacity(vec_len);

        bytes.push(self.count);

        for fh in &self.content {
            bytes.extend(fh.serialize());
        }

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < OFFSET {
            return None;
        }

        let count = bytes[0];

        if bytes.len() < OFFSET + (count as usize) * FILE_HEADER_LENGTH {
            return None;
        }

        let mut content = Vec::with_capacity(count as usize);

        let content_bytes_len = count as usize * FILE_HEADER_LENGTH;
        let content_bytes: &[u8] = &bytes[OFFSET..OFFSET + content_bytes_len];

        for i in 0..(count as usize) {
            let slice_start = i*FILE_HEADER_LENGTH;
            let slice_end = (i+1) * FILE_HEADER_LENGTH;

            let slice = &content_bytes[slice_start..slice_end];
            
            if let Some(fh) = FileHeader::deserialize(slice) {
                content.push(fh);
            }
        }

        Some(Self { count, content })
    }
}