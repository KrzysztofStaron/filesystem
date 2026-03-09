#[repr(C)]
pub struct FileHeader {
    pub extension: u8,
    pub name: [u8; 16],
    pub length: u32,
    pub start: u32,
}

impl FileHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(21);
        bytes.push(self.extension);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.length.to_le_bytes());
        bytes.extend_from_slice(&self.start.to_le_bytes());
        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 25 {
            return None;
        }

        let extension = bytes[0];
        let mut name = [0u8; 16];
        name.copy_from_slice(&bytes[1..17]);
        let length = u32::from_le_bytes(bytes[17..21].try_into().unwrap());
        let start: u32 = u32::from_le_bytes(bytes[21..25].try_into().unwrap());

        Some(Self { extension, name, length, start })
    }
}