# notes

A note taking tool in the command line.  
This Program does not use *~~zero based indexing~~* but rather __one based indexing__.  
This means the first element is referenced as index 1.  

## Usage

To add a new note into the notebook:

```bash
cargo run -- -w "This is a new note"
```

To read all the notes in the notebook:

```bash
cargo run -- -r
```

To update the first record  in the notebook:

```bash
cargo run -- -u 1 "This is the updated Record"
```

To delete the first record off of the notebook:

```bash
cargo run -- -d 1 
```
