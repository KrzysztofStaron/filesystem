# filesystem

# Main Header:
 - count - 1byte to store how many files are there
 - content: ```Vec<FileHeader>``` - metadata about files

 # File header:
 - extension - 1byte for the type of file (TEXT, BINARY, IMG...)
 - name

 - length - how big is that file
 - start - where data content is saved