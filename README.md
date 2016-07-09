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

**Note:** This is a work in progress. As of 2016-07-08, there is no game to be played. Just a demonstration of the original game assets being loaded and other proof of concepts. It will be a while before it is playable.

### WIP Screenshot

![Work in Progress Screenshot](static/2016_07_08_1_OpenAOE.png?raw=true "Terrain rendering")

### Building and Running

You'll need the Rust compiler and Cargo build system. Once you have those,
you can compile with:

```
$ cargo build --release
```

Before you can run the game, you'll need to place the game's data in a place where the program can find it. On the game CD, there is a `game/data` directory with a bunch of DRS, AI, PER, and DAT files in it. Either symlink that directory into the root of the project, or copy it over (it should keep the name "data"). Once the data is placed, you can run the game with:

```
$ cargo run
```

### Contributing

OpenAOE is MIT licensed. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be as MIT, without any additional terms or conditions.

Pull requests, especially pertaining to accuracy/bug-fixes, are always more than welcome!
