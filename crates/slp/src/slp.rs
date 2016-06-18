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

use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::mem::size_of;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use quick_error::ResultExt;

quick_error! {
    #[derive(Debug)]
    pub enum SlpError {
        ReadError(err: io::Error, path: PathBuf) {
            display("failed to read SLP {:?}: {}", path, err)
            context(path: &'a Path, err: io::Error)
                -> (err, path.to_path_buf())
        }
        InvalidSlp(reason: &'static str, path: PathBuf) {
            display("invalid SLP {:?}: {}", path, reason)
        }
        CorruptSlp(reason: String, path: PathBuf) {
            display("corrupt SLP {:?}: {}", path, reason)
        }
    }
}

pub type SlpResult<T> = Result<T, SlpError>;

pub struct SlpHeader {
    pub file_version: [u8; 4],
    pub shape_count: u32,
    pub comment: [u8; 24],
}

impl SlpHeader {
    pub fn new() -> SlpHeader {
        SlpHeader {
            file_version: [0u8; 4],
            shape_count: 0u32,
            comment: [0u8; 24],
        }
    }

    // TODO: Implement writing

    fn read_from_file<R: Read + Seek>(file: &mut R, file_name: &Path) -> SlpResult<SlpHeader> {
        let mut header = SlpHeader::new();
        try!(file.read_exact(&mut header.file_version).context(file_name));
        header.shape_count = try!(file.read_u32::<LittleEndian>().context(file_name));
        try!(file.read_exact(&mut header.comment).context(file_name));

        if header.file_version[0] != '2' as u8 || header.file_version[1] != '.' as u8 ||
                header.file_version[2] != '0' as u8 || header.file_version[3] != 'N' as u8 {
            return Err(SlpError::InvalidSlp("bad header", file_name.to_path_buf()))
        }
        Ok(header)
    }
}

#[cfg(test)]
#[test]
fn test_slp_header_read_from_file() {
    use std::io::Cursor;
    let data = vec!['2' as u8, '.' as u8, '0' as u8, 'N' as u8, 4, 0, 0, 0,
                    't' as u8, 'e' as u8, 's' as u8, 't' as u8, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = SlpHeader::read_from_file(&mut Cursor::new(data), &PathBuf::from("test"));
    match result {
        Ok(slp_header) => assert_eq!(4u32, slp_header.shape_count),
        Err(e) => panic!("unexpected error: {}", e),
    }
}

#[cfg(test)]
#[test]
fn test_slp_header_read_from_file_bad_header() {
    use std::io::Cursor;
    let data = vec!['2' as u8, '.' as u8, '1' as u8, 'N' as u8, 4, 0, 0, 0,
                    't' as u8, 'e' as u8, 's' as u8, 't' as u8, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = SlpHeader::read_from_file(&mut Cursor::new(data), &PathBuf::from("test"));
    match result {
        Ok(slp_header) => panic!("expected bad header error"),
        Err(e) => match e {
            SlpError::InvalidSlp(reason, path) => assert_eq!("bad header", reason),
            _ => panic!("unexpected error: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct SlpShapeHeader {
    pub shape_data_offsets: u32,
    pub shape_outline_offset: u32,
    pub palette_offset: u32,
    pub properties: u32,
    pub width: u32,
    pub height: u32,
    pub center_x: i32,
    pub center_y: i32,
}

impl SlpShapeHeader {
    pub fn new() -> SlpShapeHeader {
        SlpShapeHeader {
            shape_data_offsets: 0u32,
            shape_outline_offset: 0u32,
            palette_offset: 0u32,
            properties: 0u32,
            width: 0u32,
            height: 0u32,
            center_x: 0i32,
            center_y: 0i32,
        }
    }

    // TODO: Implement writing

    fn read_from_file<R: Read + Seek>(file: &mut R, file_name: &Path) -> SlpResult<SlpShapeHeader> {
        let mut header = SlpShapeHeader::new();
        header.shape_data_offsets = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.shape_outline_offset = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.palette_offset = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.properties = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.width = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.height = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.center_x = try!(file.read_i32::<LittleEndian>().context(file_name));
        header.center_y = try!(file.read_i32::<LittleEndian>().context(file_name));
        Ok(header)
    }
}

pub type SlpPixels = Vec<u8>;

pub struct SlpLogicalShape {
    pub header: SlpShapeHeader,
    pub pixels: SlpPixels,
}

impl SlpLogicalShape {
    pub fn new() -> SlpLogicalShape {
        SlpLogicalShape {
            header: SlpShapeHeader::new(),
            pixels: SlpPixels::new(),
        }
    }
}

pub struct SlpFile {
    pub header: SlpHeader,
    pub shapes: Vec<SlpLogicalShape>,
}

impl SlpFile {
    pub fn new() -> SlpFile {
        SlpFile {
            header: SlpHeader::new(),
            shapes: Vec::new(),
        }
    }

    // TODO: Implement writing

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> SlpResult<SlpFile> {
        let file_name = file_name.as_ref();
        let mut file = try!(File::open(file_name).context(file_name));

        let mut slp_file = SlpFile::new();
        slp_file.header = try!(SlpHeader::read_from_file(&mut file, file_name));
        for _shape_index in 0..slp_file.header.shape_count {
            let mut shape = SlpLogicalShape::new();
            shape.header = try!(SlpShapeHeader::read_from_file(&mut file, file_name));
            slp_file.shapes.push(shape);
        }

        for shape in &mut slp_file.shapes {
            try!(SlpFile::read_pixel_data(&mut file, file_name, shape));
        }

        Ok(slp_file)
    }

    fn read_pixel_data<R: Read + Seek>(file: &mut R, file_name: &Path, shape: &mut SlpLogicalShape)
            -> SlpResult<()> {
        let width = shape.header.width;
        let height = shape.header.height;

        // Reserve and zero out pixel data
        shape.pixels.resize((width * height) as usize, 0u8);

        for y in 0..height {
            let line_outline_offset = shape.header.shape_outline_offset + (y * size_of::<u32>() as u32);

            // TODO: Debug info; remove
            //println!("Outline offset: 0x{:04X}, y={}", line_outline_offset, y);

            try!(file.seek(SeekFrom::Start(line_outline_offset as u64)).context(file_name));
            let mut x = try!(file.read_u16::<LittleEndian>().context(file_name)) as u32;
            let right_padding = try!(file.read_u16::<LittleEndian>().context(file_name)) as u32;
            if x == 0x8000 || right_padding == 0x8000 {
                // Fully transparent; skip to next line
                continue;
            }

            // The shape_data_offset points to an array of offsets to actual pixel data
            // Seek out the offset for the current Y coordinate
            let shape_data_ptr_offset = shape.header.shape_data_offsets + (y * size_of::<u32>() as u32);
            try!(file.seek(SeekFrom::Start(shape_data_ptr_offset as u64)).context(file_name));

            // Read the offset and seek to it so we can see the actual data
            let data_offset = try!(file.read_u32::<LittleEndian>().context(file_name));
            try!(file.seek(SeekFrom::Start(data_offset as u64)).context(file_name));

            // TODO: Debug info; remove
            //println!("Current offset: 0x{:04X}", data_offset);

            // TODO: Consider detecting endless loop when we loop more times than there are pixels
            loop {
                let cmd_byte = try!(read_byte(file, file_name));
                //println!("Command={:02X}  x={}:", cmd_byte, x);

                // End of line indicator
                if cmd_byte == 0x0F {
                    if x != width - right_padding {
                        return Err(SlpError::CorruptSlp(
                            format!("Line {} not the expected size. Was {} but should be {}",
                                y, x, width - right_padding), file_name.to_path_buf()));
                    }
                    break;
                }

                if x > width {
                    return Err(SlpError::CorruptSlp(
                        "Unexpected error occurred. Line length already exceeded before stop.".to_string(),
                            file_name.to_path_buf()));
                }

                match cmd_byte & 0x0F {
                    // Block copy
                    0x00 | 0x04 | 0x08 | 0x0C => {
                        let length = cmd_byte >> 2;
                        if length == 0 {
                            return Err(SlpError::CorruptSlp(
                                format!("Block copy encountered zero length at 0x{:08X}", data_offset),
                                    file_name.to_path_buf()));
                        }
                        for _ in 0..length {
                            shape.pixels[(y * width + x) as usize] =
                                    try!(read_byte(file, file_name));
                            x += 1;
                        }
                    }

                    // Skip pixels
                    0x01 | 0x05 | 0x09 | 0x0D => {
                        let length = cmd_byte >> 2;
                        if length == 0 {
                            return Err(SlpError::CorruptSlp(
                                format!("Skip pixels encountered zero length at 0x{:08X}", data_offset),
                                    file_name.to_path_buf()));
                        }
                        x += length as u32;
                    }

                    // Large block copy
                    0x02 => {
                        let mut length = ((cmd_byte & 0xF0) as usize) << 4;
                        length += try!(read_byte(file, file_name)) as usize;
                        for _ in 0..length {
                            shape.pixels[(y * width + x) as usize] =
                                    try!(read_byte(file, file_name));
                            x += 1;
                        }
                    }

                    // Large skip pixels
                    0x03 => {
                        let mut length = ((cmd_byte & 0xF0) as usize) << 4;
                        length += try!(read_byte(file, file_name)) as usize;
                        x += length as u32;
                    }

                    // Copy and colorize block
                    0x06 => {
                        let mut length = cmd_byte >> 4;
                        if length == 0 {
                            length = try!(read_byte(file, file_name));
                        }
                        for _ in 0..length {
                            // TODO: OR in the player color
                            shape.pixels[(y * width + x) as usize] =
                                    try!(read_byte(file, file_name));
                            x += 1;
                        }
                        //println!("block copied and colorized: {}", length);
                    }

                    // Fill block
                    0x07 => {
                        let mut length = cmd_byte >> 4;
                        if length == 0 {
                            length = try!(read_byte(file, file_name));
                        }
                        let color = try!(read_byte(file, file_name));
                        for _ in 0..length {
                            shape.pixels[(y * width + x) as usize] = color;
                            x += 1;
                        }
                        //println!("block filled: {}", length);
                    }

                    // Transform block
                    0x0A => {
                        let mut length = cmd_byte >> 4;
                        if length == 0 {
                            length = try!(read_byte(file, file_name));
                        }
                        // TODO: Render the shadow instead of skipping
                        // "The length is determined as in cases 6 and 7. The next byte in the
                        // stream determines the initial color of the block run, and it is
                        // and-ed to the shadow "and" mask, and then or-ed to the shadow "or"
                        // mask. These masks are typically something like 0xff00ff00 and
                        // 0x00ff00ff, and are used to draw shadow effects in the game.
                        // This is typically used to overlay a checkerboard shadow sprite onto
                        // the existing buffer." -- slp.txt

                        // Skip forward one byte for now
                        try!(file.seek(SeekFrom::Current(1i64)).context(file_name));
                        x += length as u32;
                        println!("TODO: skipped {} instead of drawing the shadow", length);
                    }

                    // Shadow pixels
                    0x0B => {
                        let mut length = cmd_byte >> 4;
                        if length == 0 {
                            length = try!(read_byte(file, file_name));
                        }
                        // TODO: Render the shadow instead of skipping
                        // The length is determined as in cases 6, 7 and 0x0a. For the length
                        // of the run, the destination pixels already in the buffer are used
                        // as a lookup into a "shadow table" and this lookup pixel is then
                        // used to draw into the buffer. The shadow table is typically a
                        // color-tinted variation of the real color table, and is generally
                        // used to draw things like the red-tinted checkerboard sprites when
                        // you try to place a building in an area where it cannot be placed.
                        x += length as u32;
                        println!("TODO: skipped {} instead of drawing the shadow", length);
                    }

                    // Extended
                    0x0E => {
                        panic!("Extended (0x0E) not implemented")
                    }

                    _ => panic!("unknown command: {}", cmd_byte)
                }
            }
        }
        Ok(())
    }
}

fn read_byte<R: Read>(file: &mut R, file_name: &Path) -> SlpResult<u8> {
    let mut buffer = [0u8; 1];
    try!(file.read_exact(&mut buffer).context(file_name));
    Ok(buffer[0])
}
