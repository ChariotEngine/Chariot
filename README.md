# Chariot [![Build Status](https://travis-ci.org/ChariotEngine/Chariot.svg?branch=master)](https://travis-ci.org/ChariotEngine/Chariot)

The Chariot project is an attempt to re-implement the original Age of Empires (1997) engine
in an open-source manner so that:

 - The game may be ported to any desired platform.
 - Useful features from the sequel, such as build queuing, can be added to bring the original game closer to modern day RTS standards.
 - Enhancements, such as larger screen resolution support, can be made.

The project will strive to be as close to the original as possible.

For legal reasons, you'll need an original Age of Empires CD to be able to
play it. No game data files will be committed to the repository.

**Note:** This is a work in progress. As of 2016-07-08, there is no game to be played. Just a demonstration of the original game assets being loaded and other proof of concepts. It will be a while before it is playable.

# WIP Screenshot

![Work in Progress Screenshot](https://cloud.githubusercontent.com/assets/20009343/16906794/daccd474-4c71-11e6-90ec-6821e5797b5c.png)

# Building and Running

## Overview

OS-specific instructions will follow. This section just gives a high level idea of what is required to build Chariot. You'll need the following:

1. [A Rust Compiler](https://www.rust-lang.org)
2. GCC (via [MINGW](http://www.mingw.org/) if on Windows), or [Microsoft Visual C++](https://www.visualstudio.com/en-us/visual-studio-homepage-vs.aspx)
3. [LibSDL2](https://www.libsdl.org/)

Rust's Cargo program should download and compile all of the other necessary dependencies.

## Building on Linux

1. Install Rust. Documentation for [manual install here](https://doc.rust-lang.org/book/getting-started.html).
2. Install SDL2. If you're on Ubuntu, you can use this command: `sudo apt-get install libsdl2-dev`.
3. Install GCC. For Ubuntu / Debian use the 'build-essential' package. For Arch linux use 'base-devel'
3. Build the game with: `cargo build --release`

## Building on Windows

### MSVC

1. Install the Visual C++ 2015 Build Tools (make sure to choose the default installation, as custom installation has been known to cause problems).
2. Install Rust via [rustup](https://www.rustup.rs/).
3. Follow the [Windows (MSVC)](https://github.com/AngryLawyer/rust-sdl2#windows-msvc) instructions for Rust SDL2 bindings.
4. Build the game with: `cargo build --release`.

### MinGW/MSYS2

1. Install Rust via [rustup](https://www.rustup.rs/).
2. Install [MinGW/MSYS2](http://msys2.github.io/)
3. Install SDL2: `pacman -S mingw-w64-x86_64-SDL2`
4. Export the library folder: `echo "export LIBRARY_PATH=/usr/local/lib/:/lib/" >> /etc/profile"`
5. Use the GNU ABI (`i686-pc-windows-gnu` or `x86_64-windows-pc-gnu`)
6. Build the game with: `cargo build --release`

## Building on macOS

1. Install [Homebrew](http://brew.sh/)
2. Install Rust. `curl https://sh.rustup.rs -sSf | sh && rustup install toolchain stable-x86_64-apple-darwin`
3. Install SDL2. `brew install sdl2`
4. Build the game with: `make build` (which invokes `cargo build --release`)

## Running

On the game CD you will find a `GAME` directory with a `LANGUAGE.DLL`, `EMPIRES.EXE`, and a bunch of directories such as `AVI`, `CAMPAIGN`, and `DATA`.

Substitute `/media/AOE/GAME` in the following commands with the absolute path to the `GAME` directory mentioned above.

The following commands must be executed from the root of this project (the same directory that contains `Makefile`).

```sh
$ make run GAME_DIR=/media/AOE/GAME SCENARIO=MUF7E5_1

# Or you can run cargo directly:
$ cargo run --release -- /media/AOE/GAME/SCENARIO/MUF7E5_1.SCN --game-data-dir /media/AOE/GAME
```

Note that in these early versions, you may need to specify additional command line arguments, such as a path to a scenario file to load up. These may change over time, but the game should tell you what arguments are required and what to provide.

# Contributing

Chariot is MIT licensed.

Any contribution submitted for inclusion in the work by you shall also be licensed as MIT, without any additional terms or conditions.

## IRC

We have an IRC channel setup at `#openaoe` (I'd like to move this but there are some idle lurkers) on Freenode. Most collaborative discussions take place there, so it's a good place to ask where you can help out, or how something should be approached.

## Before submitting a pull request

1. Make sure you've run the tests: `./all-crates-do test`
2. Format any source files you modified with [Rustfmt](https://github.com/rust-lang-nursery/rustfmt).
