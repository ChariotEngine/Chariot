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

extern crate minifb;
extern crate clap;

use clap::{App, Arg};

use std::io;
use std::process;
use std::thread;
use std::time::Duration;
use std::cmp;

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
            frame.buffer[dest_index] = (color.r as u32) << 16 | (color.g as u32) << 8 |
                                       (color.b as u32);
        }
    }

    frame
}

fn main() {
    let matches = App::new("slp_viewer")
        .version("1.0")
        .author("Kevin Fuller <angered.ghandi@gmail.com>")
        .about("Shows SLP files from Age of Empires (1997)")
        .arg(Arg::with_name("DRS")
            .short("d")
            .long("drs")
            .value_name("DRS")
            .help("Sets which DRS file to load the SLP from")
            .takes_value(true))
        .arg(Arg::with_name("INTERFAC")
            .short("i")
            .long("interfac")
            .value_name("INTERFAC")
            .help("Sets location for interfac.drs (to get the palette from)")
            .takes_value(true))
        .arg(Arg::with_name("SLP")
            .help("SLP ID")
            .required(true)
            .index(1))
        .get_matches();

    let slp_id = matches.value_of("SLP").unwrap().parse::<u32>().expect("valid SLP ID");
    let drs_name = matches.value_of("DRS").unwrap_or("game/data/graphics.drs");
    let interfac_name = matches.value_of("INTERFAC").unwrap_or("game/data/interfac.drs");

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

    println!("Loading SLP: {}", slp_id);
    let slp = match slp::SlpFile::read_from(&mut io::Cursor::new(slp_contents)) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to read SLP: {}", err);
            process::exit(1);
        }
    };

    println!("Loading palette");
    let bin_table = &interfac_drs.tables[0];
    let palette_contents = &bin_table.contents[26];
    let palette = match palette::read_from(&mut io::Cursor::new(palette_contents)) {
        Ok(palette) => palette,
        Err(err) => {
            println!("Failed to read palette: {}", err);
            process::exit(1);
        }
    };

    let mut frame_index: usize = 0;
    let mut current_frame = get_frame(&slp, &palette, frame_index);

    let mut window = match minifb::Window::new("slp_viewer",
                                               current_frame.width,
                                               current_frame.height,
                                               minifb::WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Failed to create window: {}", err);
            process::exit(1);
        }
    };

    while window.is_open() {
        window.update_with_buffer(&current_frame.buffer);
        thread::sleep(Duration::from_millis(100));

        let previous_frame_index = frame_index;
        if window.is_key_down(minifb::Key::Right) {
            frame_index += 1;
        } else if window.is_key_down(minifb::Key::Left) {
            frame_index = frame_index.saturating_sub(1);
        }
        frame_index = cmp::max(0, cmp::min(slp.shapes.len() - 1, frame_index));
        if previous_frame_index != frame_index {
            println!("Frame index: {}", frame_index);
            current_frame = get_frame(&slp, &palette, frame_index);
        }
    }
}
