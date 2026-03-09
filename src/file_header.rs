pub const FILE_HEADER_LENGTH: usize = 25;

#[derive(Clone, Debug)]
pub enum Extension {
    Text,
    Binary,
    Unknown(u8),
}

impl Extension {
    pub fn to_u8(&self) -> u8 {
        match self {
            Extension::Text => 1,
            Extension::Binary => 2,
            Extension::Unknown(b) => *b,
        }
    }

    pub fn from_u8(b: u8) -> Self {
        match b {
            1 => Extension::Text,
            2 => Extension::Binary,
            other => Extension::Unknown(other),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FileHeader {
    pub extension: Extension,
    pub name: [u8; 16],
    pub length: u32,
    pub start: u32,
}

impl FileHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(25);

        bytes.push(self.extension.to_u8());
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.length.to_le_bytes());
        bytes.extend_from_slice(&self.start.to_le_bytes());
        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 25 {
            return None;
        }

        let extension: Extension = Extension::from_u8(bytes[0]);
        let mut name: [u8; 16] = [0u8; 16];
        name.copy_from_slice(&bytes[1..17]);
        let length: u32 = u32::from_le_bytes(bytes[17..21].try_into().unwrap());
        let start: u32 = u32::from_le_bytes(bytes[21..25].try_into().unwrap());

        Some(Self { extension, name, length, start })
    }
}