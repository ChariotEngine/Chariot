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

use io_tools::ReadExt;

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

#[derive(Default, Debug)]
struct GraphicAttackSound {
    sound_delay: i16,
    sound_id: i16,
}

impl GraphicAttackSound {
    pub fn new() -> GraphicAttackSound {
        Default::default()
    }
}

#[derive(Default, Debug)]
struct GraphicDelta {
    graphic_id: i16,
    direction_x: u16,
    direction_y: u16,
    display_angle: i16,
}

impl GraphicDelta {
    pub fn new() -> GraphicDelta {
        Default::default()
    }
}

#[derive(Default, Debug)]
struct Graphic {
    name: String,
    short_name: String,
    slp_resource_id: i32,
    layer: u8,
    player_color: i8, // TODO: What is this for?
    second_player_color: i8, // TODO: What is this for?
    replay: u8, // TODO: What is this for?
    coordinates: [u16; 4], // TODO: What are these for?
    sound_id: i16,
    frame_count: u16,
    angle_count: u16,
    new_speed: f32, // TODO: What is this for?
    frame_rate: f32,
    replay_delay: f32,
    sequence_type: u8, // TODO: What is this for?
    id: u16, // TODO: What is this for?
    mirror_mode: u8, // TODO: What is this for?
    deltas: Vec<GraphicDelta>,
    attack_sounds: Vec<GraphicAttackSound>,
}

impl Graphic {
    pub fn new() -> Graphic {
        Default::default()
    }
}

#[derive(Default, Debug)]
struct SoundEffect {
    file_name: String,
    resource_id: i32,
    probability: u16,
}

impl SoundEffect {
    pub fn new() -> SoundEffect {
        Default::default()
    }
}

#[derive(Default, Debug)]
struct SoundEffectGroup {
    id: u16,
    play_at_update_count: u16,
    cache_time: u32,
    sound_effects: Vec<SoundEffect>,
}

impl SoundEffectGroup {
    pub fn new() -> SoundEffectGroup {
        Default::default()
    }
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct EmpiresDb {
    terrain_restriction_count: u16,
    terrain_count: u16,

    player_colors: Vec<PlayerColor>,
    sound_effect_groups: Vec<SoundEffectGroup>,
    graphics: Vec<Graphic>,
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
        Ok(db)
    }

    fn read_graphics<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let graphic_count = try!(cursor.read_u16());

        let mut graphic_pointers = Vec::new();
        for _ in 0..graphic_count {
            // No clue what these are pointing to (the numbers are too large to be file offsets),
            // but we need to skip a graphic if one of these pointers is zero
            graphic_pointers.push(try!(cursor.read_u32()));
        }

        for graphic_pointer in &graphic_pointers {
            if *graphic_pointer == 0 {
                continue;
            }

            let mut graphic = Graphic::new();
            graphic.name = try!(cursor.read_sized_str(21));
            graphic.short_name = try!(cursor.read_sized_str(13));
            graphic.slp_resource_id = try!(cursor.read_i32());

            try!(cursor.seek(SeekFrom::Current(2))); // skip 2 unknown bytes
            graphic.layer = try!(cursor.read_byte());

            graphic.player_color = try!(cursor.read_byte()) as i8;
            graphic.second_player_color = try!(cursor.read_byte()) as i8;
            graphic.replay = try!(cursor.read_byte());

            for i in 0..4 {
                graphic.coordinates[i] = try!(cursor.read_u16());
            }

            let delta_count = try!(cursor.read_u16());
            graphic.sound_id = try!(cursor.read_i16());
            let attack_sound_used = try!(cursor.read_byte());
            graphic.frame_count = try!(cursor.read_u16());
            graphic.angle_count = try!(cursor.read_u16());
            graphic.new_speed = try!(cursor.read_f32());
            graphic.frame_rate = try!(cursor.read_f32());
            graphic.replay_delay = try!(cursor.read_f32());
            graphic.sequence_type = try!(cursor.read_byte());
            graphic.id = try!(cursor.read_u16());
            graphic.mirror_mode = try!(cursor.read_byte());

            for _ in 0..delta_count {
                let mut delta = GraphicDelta::new();
                delta.graphic_id = try!(cursor.read_i16());
                try!(cursor.seek(SeekFrom::Current(6))); // skip unknown bytes
                delta.direction_x = try!(cursor.read_u16());
                delta.direction_y = try!(cursor.read_u16());
                delta.display_angle = try!(cursor.read_i16());
                try!(cursor.seek(SeekFrom::Current(2))); // skip unknown bytes
                graphic.deltas.push(delta);
            }

            if attack_sound_used != 0 {
                for _ in 0..graphic.angle_count {
                    // three sounds per angle
                    for _ in 0..3 {
                        let mut attack_sound = GraphicAttackSound::new();
                        attack_sound.sound_delay = try!(cursor.read_i16());
                        attack_sound.sound_id = try!(cursor.read_i16());
                        graphic.attack_sounds.push(attack_sound);
                    }
                }
            }
            self.graphics.push(graphic);
        }
        Ok(())
    }

    fn read_sounds<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let sound_count = try!(cursor.read_u16());
        for _ in 0..sound_count {
            let mut sound_group = SoundEffectGroup::new();
            sound_group.id = try!(cursor.read_u16());
            sound_group.play_at_update_count = try!(cursor.read_u16());

            let effect_count = try!(cursor.read_u16());
            sound_group.cache_time = try!(cursor.read_u32());

            for _ in 0..effect_count {
                let mut effect = SoundEffect::new();
                effect.file_name = try!(cursor.read_sized_str(13));
                effect.resource_id = try!(cursor.read_i32());
                effect.probability = try!(cursor.read_u16());
                sound_group.sound_effects.push(effect);
            }
            self.sound_effect_groups.push(sound_group);
        }
        Ok(())
    }

    fn read_player_colors<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let color_count = try!(cursor.read_u16());
        for _ in 0..color_count {
            let mut color = PlayerColor::new();
            color.name = try!(cursor.read_sized_str(30));
            color.id = try!(cursor.read_u16());
            try!(cursor.read_u16()); // unknown; skip

            color.palette_index = try!(cursor.read_byte());
            try!(cursor.read_byte()); // unknown byte

            self.player_colors.push(color);
        }

        Ok(())
    }

    fn read_terrain_restrictions<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        self.terrain_restriction_count = try!(cursor.read_u16());
        self.terrain_count = try!(cursor.read_u16());

        let mut terrain_restriction_pointers = Vec::new();
        for _ in 0..self.terrain_restriction_count {
            terrain_restriction_pointers.push(cursor.read_u32());
        }

        // Don't know what any of the terrain restriction data is for yet, so read/skip for now
        for _ in 0..self.terrain_restriction_count {
            for _ in 0..self.terrain_count {
                try!(cursor.read_f32()); // passable/buildable/dmg multiplier?
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
