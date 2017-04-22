// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
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

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;
extern crate open_aoe_palette as palette;

extern crate clap;
extern crate lodepng;

use clap::{App, Arg};

use std::io;
use std::process;

fn load_drs(file_name: &str) -> drs::DrsFile {
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
    buffer: Vec<u32>,
}

fn get_frame(slp: &slp::SlpFile, palette: &palette::Palette, frame_index: usize) -> Frame {
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
        buffer: vec![0; width * height],
    };

    let shape = &slp.shapes[frame_index];
    for y in 0..(shape.header.height as usize) {
        for x in 0..(shape.header.width as usize) {
            let src_index = y * (shape.header.width as usize) + x;
            let palette_index = shape.pixels[src_index];
            let color = &palette[palette_index as usize];

            let dest_index = y * frame.width + x;
            frame.buffer[dest_index] = 255 << 24 |
                                       (color.b as u32) << 16 |
                                       (color.g as u32) << 8 |
                                       (color.r as u32);
        }
    }

    frame
}

fn main() {
    let matches = App::new("slp2png")
        .version("1.0")
        .author("Taryn Hill <Phrohdoh@gmail.com>")
        .about("Converts SLP files from Age of Empires (1997) to 32-bit RGB PNGs (1 per frame)")
        .arg(Arg::with_name("DRS")
            .short("d")
            .long("drs")
            .value_name("DRS")
            .help("Filepath for DRS file to load the SLP from")
            .takes_value(true))
        .arg(Arg::with_name("INTERFAC")
            .short("i")
            .long("interfac")
            .value_name("INTERFAC")
            .help("Filepath for interfac.drs (to get the palette from)")
            .takes_value(true))
        .arg(Arg::with_name("PLAYER")
            .short("p")
            .long("player")
            .value_name("PLAYER")
            .help("Sets the player color base index (must be in range 1 to 8, inclusive)")
            .takes_value(true))
        .arg(Arg::with_name("SLP")
            .help("SLP ID")
            .required(true)
            .index(1))
        .get_matches();

    let slp_id = matches.value_of("SLP").unwrap().parse::<u32>().expect("valid SLP ID");
    let drs_name = matches.value_of("DRS").unwrap_or("game/data/graphics.drs");
    let interfac_name = matches.value_of("INTERFAC").unwrap_or("game/data/interfac.drs");
    let mut player_index = matches.value_of("PLAYER")
        .unwrap_or("1")
        .parse::<u8>()
        .expect("valid player index in the range of 1 to 8 inclusive");

    if player_index > 8 {
        player_index = 8;
    } else if player_index == 0 {
        player_index = 1;
    }

    let slp_drs = load_drs(drs_name);
    let interfac_drs = load_drs(interfac_name);

    let slp_table = slp_drs.find_table(drs::DrsFileType::Slp)
        .expect(&format!("failed to find slp table in {}", drs_name));

    let slp_contents = match slp_table.find_file_contents(slp_id) {
        Some(contents) => contents,
        None => {
            println!("Couldn't find an SLP with ID {} in {}", slp_id, drs_name);
            process::exit(1);
        }
    };

    let slp = match slp::SlpFile::read_from(&mut io::Cursor::new(slp_contents), player_index) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to read SLP: {}", err);
            process::exit(1);
        }
    };

    let bin_table = &interfac_drs.tables[0];
    let palette_contents = &bin_table.contents[26];
    let palette = match palette::read_from(&mut io::Cursor::new(palette_contents)) {
        Ok(palette) => palette,
        Err(err) => {
            println!("Failed to read palette: {}", err);
            process::exit(1);
        }
    };

    for frame_index in 0..slp.header.shape_count {
        let current_frame = get_frame(&slp, &palette, frame_index as usize);
        match lodepng::encode32_file(format!("{}.png", frame_index), &current_frame.buffer, current_frame.width, current_frame.height) {
            Ok(_) => { },
            Err(err) => {
                println!("Failed to write 32bit PNG: {}", err);
                process::exit(1);
            }
        }
    }
}
