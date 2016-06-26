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

use empires::EmpiresDb;
use error::*;

use io_tools::*;

use std::io::prelude::*;
use std::io::SeekFrom;

#[derive(Default, Debug)]
pub struct GraphicAttackSound {
    sound_delay: i16,
    sound_id: i16,
}

impl GraphicAttackSound {
    pub fn new() -> GraphicAttackSound {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct GraphicDelta {
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
pub struct Graphic {
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

impl EmpiresDb {
    pub fn read_graphics<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
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
            graphic.layer = try!(cursor.read_u8());

            graphic.player_color = try!(cursor.read_i8());
            graphic.second_player_color = try!(cursor.read_i8());
            graphic.replay = try!(cursor.read_u8());

            for i in 0..4 {
                graphic.coordinates[i] = try!(cursor.read_u16());
            }

            let delta_count = try!(cursor.read_u16());
            graphic.sound_id = try!(cursor.read_i16());
            let attack_sound_used = try!(cursor.read_u8());
            graphic.frame_count = try!(cursor.read_u16());
            graphic.angle_count = try!(cursor.read_u16());
            graphic.new_speed = try!(cursor.read_f32());
            graphic.frame_rate = try!(cursor.read_f32());
            graphic.replay_delay = try!(cursor.read_f32());
            graphic.sequence_type = try!(cursor.read_u8());
            graphic.id = try!(cursor.read_u16());
            graphic.mirror_mode = try!(cursor.read_u8());

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
}
