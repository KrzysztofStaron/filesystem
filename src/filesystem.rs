use crate::{
    BLOCK_SIZE,
    file_header::{Extension, FileHeader, FILE_HEADER_LENGTH},
    file_system_header::{FileSystemHeader, HEADER_OFFSET},
};

pub struct FileSystem {
    pub disk: Vec<u8>,
}

impl FileSystem {
    pub fn header(&self) -> Option<FileSystemHeader> {
        FileSystemHeader::deserialize(&self.disk)
    }

    pub fn open(&self, name: &str) -> Option<File<'_>> {
        let header = self.header()?;
        let name_bytes = name.as_bytes();

        let fh = header.content.iter().find(|f| {
            let end = f.name.iter().position(|&b| b == 0).unwrap_or(f.name.len());
            &f.name[..end] == name_bytes
        })?;

        Some(File {
            disk: &self.disk,
            file_header: fh.clone(),
            start_at: header.data_start_offset(),
        })
    }

    pub fn open_mut(&mut self, name: &str) -> Option<FileMut<'_>> {
        let header = self.header()?;
        let name_bytes = name.as_bytes();

        let (file_index, fh) = header.content.iter().enumerate().find(|(_, f)| {
            let end = f.name.iter().position(|&b| b == 0).unwrap_or(f.name.len());
            &f.name[..end] == name_bytes
        })?;

        Some(FileMut {
            disk: &mut self.disk,
            file_header: fh.clone(),
            start_at: header.data_start_offset(),
            file_index,
        })
    }

    pub fn create(&mut self, name: &str, ext: Extension) -> Option<()> {
        let mut name_bytes = [0u8; 16];
        let len = name.len().min(16);
        name_bytes[..len].copy_from_slice(&name.as_bytes()[..len]);

        let mut header = FileSystemHeader::deserialize(&self.disk)?;
        let name_bytes_ref = name.as_bytes();

        if header.content.iter().any(|f| {
            let end = f.name.iter().position(|&b| b == 0).unwrap_or(f.name.len());
            &f.name[..end] == name_bytes_ref
        }) {
            return None;
        }

        let start_block = header
            .content
            .iter()
            .map(|f| (f.start as usize) + (f.length as usize))
            .max()
            .unwrap_or(0);

        let old_data_start = header.data_start_offset();

        let new_file = FileHeader {
            extension: ext,
            name: name_bytes,
            length: 1,
            start: start_block as u32,
        };

        header.content.push(new_file);
        header.count = header.content.len() as u8;
        let new_header_size = header.calc_size();
        let new_data_start = (new_header_size + BLOCK_SIZE - 1) / BLOCK_SIZE * BLOCK_SIZE;
        let disc_size = header.disc_size as usize;

        if new_data_start > old_data_start {
            let old_data_len = disc_size - old_data_start;
            let new_data_len = disc_size - new_data_start;

            if old_data_len > new_data_len {
                return None;
            }

            let buf: Vec<u8> = self.disk[old_data_start..disc_size].to_vec();
            self.disk[new_data_start..new_data_start + buf.len()].copy_from_slice(&buf);
            self.disk[old_data_start..new_data_start].fill(0);
        }

        let serialized = header.serialize();
        self.disk[..serialized.len()].copy_from_slice(&serialized);

        Some(())
    }
}

pub struct File<'a> {
    disk: &'a [u8],
    file_header: FileHeader,
    start_at: usize,
}

pub struct FileMut<'a> {
    disk: &'a mut [u8],
    file_header: FileHeader,
    start_at: usize,
    file_index: usize,
}

impl<'a> File<'a> {
    pub fn read(&self) -> Vec<u8> {
        let start = self.start_at + (self.file_header.start as usize)*BLOCK_SIZE;
        let end = start + (self.file_header.length as usize)* BLOCK_SIZE;

        self.disk[start..end].to_vec()
    }

}

impl<'a> FileMut<'a> {
    #[allow(dead_code)]
    pub fn read(&self) -> Vec<u8> {
        let start = self.start_at + (self.file_header.start as usize) * BLOCK_SIZE;
        let end = start + (self.file_header.length as usize) * BLOCK_SIZE;
        self.disk[start..end].to_vec()
    }

    pub fn write(&mut self, data: &[u8]) -> Option<()> {
        let blocks_needed = (data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let allocated_blocks = self.file_header.length as usize;
        let our_start = self.file_header.start as usize;
        let our_end = our_start + allocated_blocks;

        let header = FileSystemHeader::deserialize(&*self.disk)?;
        let disc_size = header.disc_size as usize;

        let extra_blocks = blocks_needed.saturating_sub(allocated_blocks);

        if extra_blocks > 0 {
            let files_to_move: Vec<_> = header
                .content
                .iter()
                .enumerate()
                .filter(|(i, f)| *i != self.file_index && (f.start as usize) >= our_end)
                .map(|(i, f)| (i, f.start as usize, f.length as usize))
                .collect();

            let rightmost_end = files_to_move
                .iter()
                .map(|(_, s, len)| s + len)
                .max()
                .unwrap_or(our_end);

            let new_rightmost_end = rightmost_end + extra_blocks;
            if self.start_at + new_rightmost_end * BLOCK_SIZE > disc_size {
                return None;
            }

            for (i, file_start, file_len) in files_to_move.into_iter().rev() {
                let src_start = self.start_at + file_start * BLOCK_SIZE;
                let src_end = src_start + file_len * BLOCK_SIZE;
                let dst_start = self.start_at + (file_start + extra_blocks) * BLOCK_SIZE;

                let mut buf = vec![0u8; file_len * BLOCK_SIZE];
                buf.copy_from_slice(&self.disk[src_start..src_end]);
                self.disk[dst_start..dst_start + file_len * BLOCK_SIZE].copy_from_slice(&buf);

                let mut fh = header.content[i].clone();
                fh.start = (file_start + extra_blocks) as u32;
                let header_offset = HEADER_OFFSET + i * FILE_HEADER_LENGTH;
                self.disk[header_offset..header_offset + FILE_HEADER_LENGTH]
                    .copy_from_slice(&fh.serialize());
            }
        }

        let start = self.start_at + our_start * BLOCK_SIZE;
        let end = start + blocks_needed * BLOCK_SIZE;

        if end > self.disk.len() {
            return None;
        }

        self.disk[start..start + data.len()].copy_from_slice(data);
        self.disk[start + data.len()..end].fill(0);

        self.file_header.length = blocks_needed as u32;
        self.file_header.start = our_start as u32;

        let header_offset = HEADER_OFFSET + self.file_index * FILE_HEADER_LENGTH;
        let serialized = self.file_header.serialize();
        self.disk[header_offset..header_offset + FILE_HEADER_LENGTH].copy_from_slice(&serialized);

        Some(())
    }
}