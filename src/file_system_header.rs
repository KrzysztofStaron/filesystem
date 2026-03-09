#[repr(C)]
pub struct FileSystemHeader {
    pub files: u8,
    pub max_files: u8,
}

impl FileSystemHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2);
        bytes.push(self.files);
        bytes.push(self.max_files);

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 2 {
            return None;
        }

        let files = bytes[0];
        let max_files = bytes[1];

        if files > max_files {
            return None;
        }

        Some(Self { files, max_files })
    }
}