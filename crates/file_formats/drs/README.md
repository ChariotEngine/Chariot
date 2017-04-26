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
