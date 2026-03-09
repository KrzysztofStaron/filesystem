# filesystem

small cli to use a custom filesystem. disc is simulated by a single file (`mydisk.img`), every operation happens on that one file.

## how it works

3 components:

- **SystemHeader** – 5 bytes, basic metadata
- **FileHeaders** – 25 bytes each, metadata per file: name, extension, length, position in blob
- **Blob** – sector split into 512-byte blocks. one block = one file (files can span multiple blocks)

when a file's block count changes, the fs walks through every file on disc and moves them to free space or pack them tighter. The only reason for blocks is so small edits don't force a full re-allocation.

```bash
cargo build --release
cargo run --release
```

commands: `ls` `cat` `touch` `write` `big` `status` `resetfs` `help` `quit`
