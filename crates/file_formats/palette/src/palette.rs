// Chariot: An open source reimplementation of Age of Empires (1997)
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

use error::*;

use std::io::{BufRead, BufReader, Seek};
use std::path::Path;
use std::fs::File;

#[derive(Copy, Clone)]
pub struct PaletteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PaletteColor {
    pub fn new() -> PaletteColor {
        PaletteColor {
            r: 0u8,
            g: 0u8,
            b: 0u8,
        }
    }
}

impl Into<u32> for PaletteColor {
    /// Converts PaletteColor into RGBA8888 u32 format
    fn into(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | 0xFF
    }
}

pub type Palette = Vec<PaletteColor>;

pub fn read_from<R: BufRead + Seek>(file: &mut R) -> Result<Palette> {
    let mut palette = Palette::new();

    let mut line_index = 0;
    for line_result in file.lines() {
        let line = try!(line_result);
        if line_index == 0 && line != "JASC-PAL" || line_index == 1 && line != "0100" ||
           line_index == 2 && line != "256" {
            return Err(ErrorKind::InvalidPalette("bad header").into());
        }
        if line_index > 2 {
            let components: Vec<&str> = line.split_whitespace().collect();
            if components.len() != 3 {
                return Err(ErrorKind::InvalidPalette("invalid color found").into());
            }
            let mut color = PaletteColor::new();
            color.r = try!(components[0].parse::<u8>());
            color.g = try!(components[1].parse::<u8>());
            color.b = try!(components[2].parse::<u8>());
            palette.push(color);
        }
        line_index += 1;
    }

    Ok(palette)
}

pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> Result<Palette> {
    let f = File::open(file_path)?;
    let mut br = BufReader::new(f);
    read_from(&mut br)
}