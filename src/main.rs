use std::io::{self, BufRead, Read, Write};

mod file_header;
use file_header::Extension;

mod filesystem;
use filesystem::FileSystem;

mod file_system_header;
use file_system_header::FileSystemHeader;

mod utils;

pub const BLOCK_SIZE: usize = 512;
pub const DISC_SIZE_BLOCKS: usize = 2048;
pub const DISC_SIZE_BYTES: usize = BLOCK_SIZE * DISC_SIZE_BLOCKS;
pub const DISC_NAME: &str = "mydisk.img";

fn main() {
    if !std::path::Path::new(DISC_NAME).exists() {
        init_empty_disk();
    }

    run_terminal();
}

fn run_terminal() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(stdout, "$ ").unwrap();
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mut fs = FileSystem { disk: read_disk() };
        match parts[0] {
            "ls" => cmd_ls(&fs),
            "cat" => {
                if parts.len() < 2 {
                    println!("cat: missing filename");
                } else {
                    cmd_cat(&fs, parts[1]);
                }
            }
            "touch" => {
                if parts.len() < 2 {
                    println!("touch: missing filename");
                } else {
                    cmd_create(&mut fs, parts[1]);
                }
            }
            "write" => {
                if parts.len() < 3 {
                    println!("write: usage: write FILE TEXT");
                } else {
                    let content = parts[2..].join(" ");
                    cmd_write(&mut fs, parts[1], &content);
                }
            }
            "big" => {
                if parts.len() < 3 {
                    println!("big: usage: big FILE SIZEINBYTES");
                } else {
                    match parts[2].parse::<usize>() {
                        Ok(size) => cmd_big(&mut fs, parts[1], size),
                        Err(_) => println!("big: invalid size: {}", parts[2]),
                    }
                }
            }
            "help" => cmd_help(),
            "status" => cmd_status(&fs),
            "resetfs" => cmd_resetfs(&mut fs),
            "exit" | "quit" => break,
            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}

fn cmd_create(fs: &mut FileSystem, filename: &str) {
    let ext = if filename.ends_with(".txt") {
        Extension::Text
    } else if filename.ends_with(".bin") {
        Extension::Binary
    } else {
        Extension::Text
    };

    match fs.create(filename, ext) {
        Some(()) => {}
        None => println!("touch: {}: file exists or error", filename),
    }
    save_disk(fs);
}

fn cmd_big(fs: &mut FileSystem, filename: &str, size: usize) {
    const LOREM: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let content: Vec<u8> = LOREM.iter().cycle().take(size).copied().collect();

    if fs.open(filename).is_none() {
        let ext = if filename.ends_with(".txt") {
            Extension::Text
        } else if filename.ends_with(".bin") {
            Extension::Binary
        } else {
            Extension::Text
        };
        if fs.create(filename, ext).is_none() {
            println!("big: {}: could not create file", filename);
            return;
        }
    }

    let Some(mut file) = fs.open_mut(filename) else {
        println!("big: {}: no such file", filename);
        return;
    };
    match file.write(&content) {
        Some(()) => println!("Created {} ({} bytes)", filename, size),
        None => println!("big: {}: not enough space", filename),
    }
    save_disk(fs);
}

fn cmd_write(fs: &mut FileSystem, filename: &str, content: &str) {
    let Some(mut file) = fs.open_mut(filename) else {
        println!("write: {}: no such file", filename);
        return;
    };
    match file.write(content.as_bytes()) {
        Some(()) => {}
        None => println!("write: {}: not enough space", filename),
    }
    save_disk(fs);
}

fn save_disk(fs: &FileSystem) {
    std::fs::write(DISC_NAME, &fs.disk).unwrap();
}

fn cmd_help() {
    println!("Available commands:");
    println!("  ls                    list files");
    println!("  cat FILE              display file contents");
    println!("  touch FILE            create a new file");
    println!("  write FILE TEXT       write text to a file");
    println!("  big FILE SIZE         create file with lorem ipsum (SIZE in bytes)");
    println!("  status                show disc usage and debug info");
    println!("  resetfs               clear the filesystem");
    println!("  help                  show this help");
    println!("  exit, quit            exit the terminal");
}

fn cmd_status(fs: &FileSystem) {
    let Some(header) = fs.header() else {
        println!("status: invalid filesystem");
        return;
    };

    let disc_size = header.disc_size as usize;
    let header_size = header.calc_size();
    let data_start = header.data_start_offset();
    let data_region_size = disc_size - data_start;
    let total_blocks = data_region_size / BLOCK_SIZE;

    let used_blocks: usize = header.content.iter().map(|f| f.length as usize).sum();
    let used_bytes = used_blocks * BLOCK_SIZE;
    let free_blocks = total_blocks.saturating_sub(used_blocks);
    let free_bytes = free_blocks * BLOCK_SIZE;

    println!("--- Disc status ---");
    println!("Total size:     {} ({} bytes)", utils::format_bytes(disc_size as u64), disc_size);
    println!("Used:           {} ({} bytes, {} blocks)", utils::format_bytes(used_bytes as u64), used_bytes, used_blocks);
    println!("Available:      {} ({} bytes, {} blocks)", utils::format_bytes(free_bytes as u64), free_bytes, free_blocks);
    println!();
    println!("--- Debug info ---");
    println!("Header size:    {} bytes", header_size);
    println!("Data start:     {} ", data_start);
    println!("File count:    {}", header.count);
    println!();
    println!("--- Files ---");
    for (i, f) in header.content.iter().enumerate() {
        let name = std::str::from_utf8(&f.name).unwrap_or("?").trim_end_matches('\0');
        let size = (f.length as usize) * BLOCK_SIZE;
        println!("  [{}] {}  start={}  length={} blocks  size={} bytes", i, name, f.start, f.length, size);
    }
}

fn cmd_resetfs(fs: &mut FileSystem) {
    init_empty_disk();
    *fs = FileSystem { disk: read_disk() };
    println!("Filesystem reset.");
}

fn cmd_ls(fs: &FileSystem) {
    let Some(header) = fs.header() else {
        println!("ls: invalid filesystem");
        return;
    };

    for file in &header.content {
        let name = std::str::from_utf8(&file.name)
            .unwrap_or("<invalid>")
            .trim_end_matches('\0');
        println!("{}", name);
    }
}

fn cmd_cat(fs: &FileSystem, filename: &str) {
    let Some(file) = fs.open(filename) else {
        println!("cat: {}: no such file", filename);
        return;
    };
    let content = file.read();
    let len = content.iter().position(|&b| b == 0).unwrap_or(content.len());
    if let Ok(s) = std::str::from_utf8(&content[..len]) {
        println!("{}", s);
    }
}

fn init_empty_disk() {
    let file_system_header = FileSystemHeader {
        count: 0,
        disc_size: DISC_SIZE_BYTES as u32,
        content: vec![],
    };

    let mut bytes = file_system_header.serialize();
    bytes.resize(DISC_SIZE_BYTES, 0);

    let mut f = std::fs::File::create(DISC_NAME).unwrap();
    f.write_all(&bytes).unwrap();
    drop(f);
}

fn read_disk() -> Vec<u8> {
    let mut f = std::fs::File::open(DISC_NAME).unwrap();
    let mut bytes = Vec::new();

    f.read_to_end(&mut bytes).unwrap();
    drop(f);

    bytes
}