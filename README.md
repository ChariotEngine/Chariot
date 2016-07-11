# OpenAOE [![Build Status](https://travis-ci.org/angered-ghandi/OpenAOE.svg?branch=master)](https://travis-ci.org/angered-ghandi/OpenAOE)

The OpenAOE project is an attempt to re-implement the original Age of Empires (1997)
in an open-source manner so that:

 - The game may be ported to any desired platform.
 - Useful features from the sequel, such as build queuing, can be added to bring the original game closer to modern day RTS standards.
 - Enhancements, such as larger screen resolution support, can be made.

The OpenAOE project will strive to be as close to the original as possible.

For obvious reasons, you'll need an original Age of Empires CD to be able to
play it. No game data files will be committed to the repository.

**Note:** This is a work in progress. As of 2016-07-08, there is no game to be played. Just a demonstration of the original game assets being loaded and other proof of concepts. It will be a while before it is playable.

# WIP Screenshot

![Work in Progress Screenshot](static/2016_07_08_1_OpenAOE.png?raw=true "Terrain rendering")

# Building and Running

## Overview

OS-specific instructions will follow. This section just gives a high level idea of what is required to build OpenAOE. You'll need the following:

1. [A Rust Compiler](https://www.rust-lang.org)
2. GCC (via [MINGW](http://www.mingw.org/) if on Windows), or [Microsoft Visual C++](https://www.visualstudio.com/en-us/visual-studio-homepage-vs.aspx)
3. [LibSDL2](https://www.libsdl.org/)

Rust's Cargo program should download and compile all of the other necessary dependencies.

## Building on Linux

1. Install Rust. Documentation for [manual install here](https://doc.rust-lang.org/book/getting-started.html).
2. Install SDL2. If you're on Ubuntu, you can use this command: `sudo apt-get install libsdl2-dev`.
3. Build the game with: `cargo build --release`

## Building on Windows

(Documentation contribution appreciated)

## Building on Mac OS

(Documentation contribution appreciated)

## Setting up the Game Data

Before you can run the game, you'll need to place the game's data in a place where the program can find it. On the game CD, there is a `game` directory with a language.dll, empires.exe, and a bunch of directories such as avi, campaign, and data. Either symlink that directory into the root of the project, or copy it over (it should keep the name "game"). Once the data is placed, you can run the game with:

```
$ cargo run --release
```

Alternatively, you can specify the location of the game data via command-line:

```
$ cargo run --release -- -d /media/AOE/game
```

Note that in these early versions, you may need to specify additional command line arguments, such as a path to a scenario file to load up. These may change over time, but the game should tell you what arguments are required and what to provide.

# Contributing

OpenAOE is MIT licensed. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be as MIT, without any additional terms or conditions.

Pull requests, especially pertaining to accuracy/bug-fixes, are always more than welcome!
