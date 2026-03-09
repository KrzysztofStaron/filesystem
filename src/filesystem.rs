use crate::{file_header::Extension, file_system_header::FileSystemHeader};

pub struct FileSystem {
    pub disk: Vec<u8>,
}

impl FileSystem {
    pub fn header(&self) -> Option<FileSystemHeader> {
        FileSystemHeader::deserialize(&self.disk)
    }

    pub fn open(&self, name: &str){

    }

    pub fn create(&mut self, name: &str, ext: Extension) {

    }
}

pub struct File {

}

impl File {
    pub fn read(&self) {

    }

    pub fn write(&mut self, data: &[u8]) {

    }
}