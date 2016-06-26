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

mod age;
mod civ;
mod graphics;
mod player_color;
mod random_map;
mod sound;
mod terrain_block;
mod terrain_restrictions;

use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use flate2::Decompress;
use flate2::Flush;

use super::error::*;
use empires::graphics::*;
use empires::player_color::*;
use empires::sound::*;
use empires::terrain_block::*;
use empires::random_map::*;
use empires::age::*;
use empires::civ::*;

const EXPECTED_FILE_VERSION: &'static str = "VER 3.7\0";
const DECOMPRESSION_CHUNK_SIZE: usize = 16 * 1024; // 16 kibibytes

#[derive(Default, Debug)]
pub struct EmpiresDb {
    pub terrain_restriction_count: u16,
    pub terrain_count: u16,

    pub player_colors: Vec<PlayerColor>,
    pub sound_effect_groups: Vec<SoundEffectGroup>,
    pub graphics: Vec<Graphic>,
    pub terrain_block: TerrainBlock,
    pub random_maps: Vec<RandomMap>,
    pub ages: Vec<Age>,
    pub civilizations: Vec<Civilization>,
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
        try!(db.read_sounds(&mut cursor));
        try!(db.read_graphics(&mut cursor));
        try!(db.read_terrain_block(&mut cursor));
        try!(db.read_random_maps(&mut cursor));
        try!(db.read_ages(&mut cursor));
        try!(db.read_civs(&mut cursor));
        Ok(db)
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
