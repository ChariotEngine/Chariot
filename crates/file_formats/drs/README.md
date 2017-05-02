Chariot DRS Crate
-----------------

This crate handles the DRS archive file format used by Age of Empires (1997).
Currently, it can read DRS files, and includes an example that can be used to
extract DRS archives.

While the ability to write a DRS file is a nice to have, it's not strictly
necessary for the rest of the Chariot project, and thus, is not implemented
at this time.

The code herein falls under the same license as the rest of the Chariot project.

### Building

You'll need the Rust compiler and Cargo build system. Once you have those,
you can compile with:

```
$ cargo build
```

To build the example program that can extract DRS archives, run:

```
$ cargo build --example extract-drs
```

### Example

```rust,norun
extern crate chariot_drs as drs;

let file_name = "/path/to/archive.drs";
match drs::DrsFile::read_from_file(file_name) {
    Ok(drs_file) => {
        println!("Successfully loaded the DRS file");
        println!("Table count: {}", drs_file.header.table_count);
        for table in &drs_file.tables {
            println!("Table \"{}\":", table.header.file_extension());
            println!("  file count: {}", table.header.file_count);
        }
    },
    Err(err) => {
        println!("Failed to read the DRS file: {}", err);
    }
}
```