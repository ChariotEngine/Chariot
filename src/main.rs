//
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
//

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;
extern crate open_aoe_palette as palette;
extern crate open_aoe_dat as dat;

extern crate minifb;

use std::io;
use std::path;

fn main() {
    let graphics_drs = match drs::DrsFile::read_from_file("data/graphics.drs") {
        Ok(drs) => drs,
        Err(err) => {
            println!("Failed to read graphics.drs: {}", err);
            return;
        }
    };

    let interfac_drs = match drs::DrsFile::read_from_file("data/interfac.drs") {
        Ok(drs) => drs,
        Err(err) => {
            println!("Failed to read interfac.drs: {}", err);
            return;
        }
    };

    // Just trying to get a graphic rendered on the screen for now
    // There are a lot of things wrong with this code
    let graphics_table = graphics_drs.find_table(drs::DrsFileType::Slp).expect("slp table");
    let sample_slp_contents = graphics_table.find_file_contents(1).expect("1.slp");
    let sample_slp = match slp::SlpFile::read_from(&mut io::Cursor::new(sample_slp_contents)) {
        Ok(slp) => slp,
        Err(err) => {
            println!("Failed to read SLP: {}", err);
            return;
        }
    };

    let bin_table = &interfac_drs.tables[0];
    let palette_contents = &bin_table.contents[26];
    let palette = match palette::read_from(&mut io::Cursor::new(palette_contents),
            path::PathBuf::from("50500.bin").as_path()) {
        Ok(palette) => palette,
        Err(err) => {
            println!("Failed to read palette: {}", err);
            return;
        }
    };

    let sample_shape = &sample_slp.shapes[0];
    let width = sample_shape.header.width as usize;
    let height = sample_shape.header.height as usize;
    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let palette_index = sample_shape.pixels[index];
            let color = &palette[palette_index as usize];
            buffer[index] = (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32);
        }
    }

    let mut window = match minifb::Window::new("OpenAOE", width, height,
            minifb::WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        }
    };

    while window.is_open() {
        window.update_with_buffer(&buffer);
    }
}
