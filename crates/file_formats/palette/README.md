Chariot Palette Crate
-----------------

This crate handles the BIN file format used by Age of Empires (1997).
Currently it can read BIN files.

While the ability to write a BIN file is a nice to have, it's not strictly
necessary for the rest of the Chariot project, and thus, is not implemented
at this time.

BIN palettes are JASC palettes (used by Paintshop Pro and possibly other image manipulation software).

This library is named `chariot_palette` because I did not recall the truth above until after I published this crate.

If you want the crate name `chariot_palette` please contact me and I'll more than likely be willing to transfer the name.

I wish I had named this library `jasc_palette`. Oops.

The code herein falls under the same license as the rest of the Chariot project.

### Building

You'll need the Rust compiler and Cargo build system. Once you have those,
you can compile with:

```
$ cargo build
```