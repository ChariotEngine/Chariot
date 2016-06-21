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
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::string::FromUtf8Error;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use flate2;
use flate2::Decompress;
use flate2::Flush;

const EXPECTED_FILE_VERSION: &'static str = "VER 3.7\0";
const DECOMPRESSION_CHUNK_SIZE: usize = 16 * 1024; // 16 kibibytes

quick_error! {
    #[derive(Debug)]
    pub enum EmpiresDbError {
        ReadError(err: io::Error) {
            from()
            display("failed to read empires.dat: {}", err)
        }
        CompressionError(err: flate2::DataError) {
            from()
            display("failed to decompress empires.dat: {}", err)
        }
        BadFile(reason: &'static str) {
            display("bad empires.dat: {}", reason)
        }
        EncodingError(err: FromUtf8Error) {
            from()
            display("invalid UTF-8 encoding in empires.dat: {}", err)
        }
    }
}

pub type EmpiresDbResult<T> = Result<T, EmpiresDbError>;

#[derive(Default)]
struct PlayerColor {
    name: String,
    id: u16,
    palette_index: u8,
}

impl PlayerColor {
    fn new() -> PlayerColor {
        Default::default()
    }
}

#[derive(Default)]
pub struct EmpiresDb {
    terrain_restriction_count: u16,
    terrain_count: u16,

    player_colors: Vec<PlayerColor>,
}

impl EmpiresDb {
    fn new() -> EmpiresDb {
        Default::default()
    }

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> EmpiresDbResult<EmpiresDb> {
        let data = try!(EmpiresDb::read_and_decompress(file_name.as_ref()));
        let mut cursor = io::Cursor::new(data);

        let mut db = EmpiresDb::new();
        try!(db.read_header(&mut cursor));
        try!(db.read_terrain_restrictions(&mut cursor));
        try!(db.read_player_colors(&mut cursor));
        Ok(db)
    }

    fn read_player_colors<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let color_count = try!(cursor.read_u16::<LittleEndian>());
        for i in 0..color_count {
            println!("Reading player color {} at offset 0x{:X}",
                i, cursor.seek(SeekFrom::Current(0)).unwrap());

            let mut color = PlayerColor::new();

            let mut name_buffer = vec![0u8; 30];
            try!(cursor.read_exact(&mut name_buffer));
            color.name = try!(String::from_utf8(name_buffer));
            println!("color name: {}", color.name);

            color.id = try!(cursor.read_u16::<LittleEndian>());
            try!(cursor.read_u16::<LittleEndian>()); // unknown; skip

            let mut buffer = [0u8; 2];
            try!(cursor.read_exact(&mut buffer));
            color.palette_index = buffer[0]; // second byte is unknown
            println!("color id: {}", color.id);
            println!("color palette index: {}", color.palette_index);

            self.player_colors.push(color);
        }

        Ok(())
    }

    fn read_terrain_restrictions<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        self.terrain_restriction_count = try!(cursor.read_u16::<LittleEndian>());
        self.terrain_count = try!(cursor.read_u16::<LittleEndian>());

        println!("terrain_restriction_count: {}", self.terrain_restriction_count);
        println!("terrain_count: {}", self.terrain_count);

        let mut terrain_restriction_pointers = Vec::new();
        for _ in 0..self.terrain_restriction_count {
            terrain_restriction_pointers.push(cursor.read_u32::<LittleEndian>());
        }

        // Don't know what any of the terrain restriction data is for yet, so read/skip for now
        for _ in 0..self.terrain_restriction_count {
            for _ in 0..self.terrain_count {
                try!(cursor.read_f32::<LittleEndian>()); // passable/buildable/dmg multiplier?
            }
        }

        Ok(())
    }

    fn read_header<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let mut version = [0u8; 8];
        try!(cursor.read_exact(&mut version));
        if version != EXPECTED_FILE_VERSION.as_bytes() {
            return Err(EmpiresDbError::BadFile("unexpected file version"));
        }

        Ok(())
    }

    // The empires.dat is compressed with zlib without a header and 15 window bits
    fn read_and_decompress(file_name: &Path) -> EmpiresDbResult<Vec<u8>> {
        use flate2::Status;

        let mut file = try!(File::open(file_name));
        let mut compressed: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut compressed));

        let mut cursor = io::Cursor::new(&compressed[..]);

        // At time of implementation, the flate2 library didn't provide an easy way to
        // decompress a stream without a header, so it had to be manually implemented here
        let mut decompressed: Vec<u8> = Vec::new();
        let mut buffer = [0u8; DECOMPRESSION_CHUNK_SIZE];
        let mut decompressor = Decompress::new(false);
        loop {
            let last_out = decompressor.total_out();
            let last_in = decompressor.total_in();

            let (status, end_stream);
            {
                let input = try!(cursor.fill_buf());
                end_stream = input.is_empty();

                let flush_type = if end_stream { Flush::Finish } else { Flush::None };
                status = try!(decompressor.decompress(input, &mut buffer, flush_type));
            }

            let read = (decompressor.total_in() - last_in) as usize;
            let written = (decompressor.total_out() - last_out) as usize;

            decompressed.extend_from_slice(&buffer[0..written]);
            cursor.consume(read);

            match status {
                Status::Ok => { },
                Status::BufError if !end_stream && written == 0 => continue,
                Status::BufError | Status::StreamEnd => break,
            }
        }

        Ok(decompressed)
    }
}
