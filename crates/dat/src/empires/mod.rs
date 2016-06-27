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

#[macro_use]
mod resource;

mod age;
mod civ;
mod graphic;
mod id;
mod player_color;
mod random_map;
mod research;
mod sound;
mod terrain_block;
mod terrain_restrictions;
mod unit;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

use flate2::Decompress;
use flate2::Flush;

use empires::age::{Age, read_ages};
use empires::civ::{Civilization, read_civs};
use empires::graphic::{Graphic, read_graphics};
use empires::player_color::{PlayerColor, read_player_colors};
use empires::random_map::{RandomMap, read_random_maps};
use empires::research::{Research, read_research};
use empires::sound::{SoundEffectGroup, read_sound_effect_groups};
use empires::terrain_block::{TerrainBlock, read_terrain_block};
use empires::terrain_restrictions::{TerrainRestriction, read_terrain_restrictions};
use super::error::*;

use io_tools::*;

const EXPECTED_FILE_VERSION: &'static str = "VER 3.7\0";
const DECOMPRESSION_CHUNK_SIZE: usize = 16 * 1024; // 16 kibibytes

#[derive(Default, Debug)]
pub struct EmpiresDb {
    pub terrain_restrictions: Vec<TerrainRestriction>,
    pub player_colors: Vec<PlayerColor>,
    pub sound_effect_groups: Vec<SoundEffectGroup>,
    pub graphics: Vec<Graphic>,
    pub terrain_block: TerrainBlock,
    pub random_maps: Vec<RandomMap>,
    pub ages: Vec<Age>,
    pub civilizations: Vec<Civilization>,
    pub research: Vec<Research>,
}

impl EmpiresDb {
    fn new() -> EmpiresDb {
        Default::default()
    }

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> EmpiresDbResult<EmpiresDb> {
        let mut stream = io::Cursor::new(try!(read_and_decompress(file_name.as_ref())));

        try!(read_header(&mut stream));
        let terrain_restriction_count = try!(stream.read_u16()) as usize;
        let terrain_count = try!(stream.read_u16()) as usize;

        let mut db = EmpiresDb::new();
        db.terrain_restrictions =
            try!(read_terrain_restrictions(&mut stream, terrain_restriction_count, terrain_count));
        db.player_colors = try!(read_player_colors(&mut stream));
        db.sound_effect_groups = try!(read_sound_effect_groups(&mut stream));
        db.graphics = try!(read_graphics(&mut stream));
        db.terrain_block = try!(read_terrain_block(&mut stream));
        db.random_maps = try!(read_random_maps(&mut stream));
        db.ages = try!(read_ages(&mut stream));
        db.civilizations = try!(read_civs(&mut stream));
        db.research = try!(read_research(&mut stream));
        Ok(db)
    }
}

fn read_header<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<()> {
    let mut version = [0u8; 8];
    try!(stream.read_exact(&mut version));
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

    let mut stream = io::Cursor::new(&compressed[..]);

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
            let input = try!(stream.fill_buf());
            end_stream = input.is_empty();

            let flush_type = if end_stream { Flush::Finish } else { Flush::None };
            status = try!(decompressor.decompress(input, &mut buffer, flush_type));
        }

        let read = (decompressor.total_in() - last_in) as usize;
        let written = (decompressor.total_out() - last_out) as usize;

        decompressed.extend_from_slice(&buffer[0..written]);
        stream.consume(read);

        match status {
            Status::Ok => { },
            Status::BufError if !end_stream && written == 0 => continue,
            Status::BufError | Status::StreamEnd => break,
        }
    }

    Ok(decompressed)
}
