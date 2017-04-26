// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2017 Taryn Hill
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate chariot_drs as drs;
extern crate chariot_slp as slp;

#[macro_use(value_t)]
extern crate clap;

use clap::{App, Arg};
use std::cmp;

use std::io;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

fn load_drs(file_name: &str) -> drs::DrsFile {
    println!("Loading DRS: {}", file_name);
    match drs::DrsFile::read_from_file(file_name) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to load DRS \"{}\": {}", file_name, err);
            process::exit(1);
        }
    }
}

struct Frame {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

fn get_frame(slp: &slp::SlpFile, frame_index: usize) -> Frame {
    let (mut width, mut height) = (0usize, 0usize);
    for shape in &slp.shapes {
        if shape.header.width as usize > width {
            width = shape.header.width as usize;
        }
        if shape.header.height as usize > height {
            height = shape.header.height as usize;
        }
    }

    let mut frame = Frame {
        width: width,
        height: height,
        buffer: vec![0; width * height * 2],
    };

    let shape = &slp.shapes[frame_index];
    println!("Frame index {}: {:?}", frame_index, shape.header);
    for y in 0..(shape.header.height as usize) {
        for x in 0..(shape.header.width as usize) {
            let src_index = y * (shape.header.width as usize) + x;
            let palette_index = shape.pixels[src_index];

            let mut dest_index = y * frame.width + x;
            if dest_index % 2 != 0 {
                dest_index += 1;
            }

            frame.buffer[dest_index] = palette_index;
            frame.buffer[dest_index+1] = 0;
        }
    }

    frame
}

fn main() {
    let matches = App::new("slp_to_rg8ui")
        .version("1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Convert SLPs to an OpenGL-friendly binary blob format (rg8ui)")
        .arg(Arg::with_name("drs")
            .long("drs")
            .value_name("drs")
            .help("Sets which DRS file to load the SLP from")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("player")
            .long("player")
            .value_name("player")
            .help("Sets the player color base index (must be in range 1 to 8, inclusive)")
            .takes_value(true))
        .arg(Arg::with_name("slp")
            .long("slp-id")
            .required(true)
            .takes_value(true))
        .get_matches();

    let slp_id = value_t!(matches, "slp", u32).expect("valid SLP ID (u32)");
    let drs_name = matches.value_of("drs").unwrap();
    let mut player_index = matches.value_of("player")
        .unwrap_or("1")
        .parse::<u8>()
        .expect("valid player index in the range of 1 to 8 inclusive");

    if player_index > 8 {
        player_index = 8;
    } else if player_index == 0 {
        player_index = 1;
    }

    let slp_drs = load_drs(drs_name);

    let slp_table = slp_drs.find_table(drs::DrsFileType::Slp)
        .expect(&format!("failed to find slp table in {}", drs_name));

    let slp_contents = match slp_table.find_file_contents(slp_id) {
        Some(contents) => contents,
        None => {
            println!("Couldn't find an SLP with ID {} in {}", slp_id, drs_name);
            process::exit(1);
        }
    };

    println!("Loading SLP: {}", slp_id);
    let slp = match slp::SlpFile::read_from(&mut io::Cursor::new(slp_contents), player_index) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to read SLP: {}", err);
            process::exit(1);
        }
    };

    let mut frame_index: usize = 0;
    let mut current_frame = get_frame(&slp, frame_index);

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("output.bin")
        .expect("Failed to open/write output.bin");

    f.write(&current_frame.buffer).expect("Failed to write out binary data to output.bin");
}

