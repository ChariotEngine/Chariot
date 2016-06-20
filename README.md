OpenAOE
-------

The OpenAOE project is an attempt to re-implement the original Age of Empires (1997)
in an open-source manner so that:

 - The game may be ported to any desired platform.
 - Useful features from the sequel, such as build queuing, can be added to bring the original game closer to modern day RTS standards.
 - Enhancements, such as larger screen resolution support, can be made.

The OpenAOE project will strive to be as close to the original as possible.

For obvious reasons, you'll need an original Age of Empires CD to be able to
play it. No game data files will be committed to the repository.

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

### Contributing

OpenAOE is MIT licensed. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be as MIT, without any additional terms or conditions.

Pull requests, especially pertaining to accuracy/bug-fixes, are always more than welcome!
