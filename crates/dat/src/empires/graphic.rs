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

use empires::id::*;
use error::*;

use io_tools::*;

use std::io::prelude::*;
use std::io::SeekFrom;

#[derive(Default, Debug)]
pub struct GraphicAttackSound {
    sound_delay: i16,
    sound_group_id: SoundGroupId,
}

#[derive(Default, Debug)]
pub struct GraphicDelta {
    graphic_id: GraphicId,
    direction_x: u16,
    direction_y: u16,
    display_angle: i16,
}

#[derive(Default, Debug)]
pub struct Graphic {
    id: GraphicId,
    name: String,
    short_name: String,
    slp_id: SlpFileId,
    layer: u8,
    player_color: i8,
    second_player_color: i8,

    /// Whether or not to replay at the end of the animation
    replay: bool,

    coordinates: Vec<u16>,

    /// Sound played when graphic is on screen
    sound_group_id: SoundGroupId,

    frame_count: u16,
    angle_count: u16,
    new_speed: f32,
    frame_rate: f32,
    replay_delay: f32,
    sequence_type: u8,
    mirror_mode: u8,

    /// Deltas this graphic would create. A delta is another graphic slot
    /// that would be joined to this one
    deltas: Vec<GraphicDelta>,

    attack_sounds: Vec<GraphicAttackSound>,
}

pub fn read_graphics<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<Graphic>> {
    let mut graphics = Vec::new();
    let graphic_count = try!(stream.read_u16()) as usize;

    // No clue what these are pointing to (the numbers are too large to be file offsets),
    // but we need to skip a graphic if one of these pointers is zero
    let graphic_pointers = try!(stream.read_array(graphic_count, |c| c.read_u32()));
    for graphic_pointer in &graphic_pointers {
        if *graphic_pointer == 0 {
            continue;
        }

        let mut graphic: Graphic = Default::default();
        graphic.name = try!(stream.read_sized_str(21));
        graphic.short_name = try!(stream.read_sized_str(13));
        graphic.slp_id = SlpFileId(try!(stream.read_i32()) as isize);

        try!(stream.seek(SeekFrom::Current(2))); // skip 2 unknown bytes
        graphic.layer = try!(stream.read_u8());

        graphic.player_color = try!(stream.read_i8());
        graphic.second_player_color = try!(stream.read_i8());
        graphic.replay = try!(stream.read_u8()) != 0;
        graphic.coordinates = try!(stream.read_array(4, |c| c.read_u16()));

        let delta_count = try!(stream.read_u16()) as usize;
        graphic.sound_group_id = SoundGroupId(try!(stream.read_i16()) as isize);
        let attack_sound_used = try!(stream.read_u8()) as usize;
        graphic.frame_count = try!(stream.read_u16());
        graphic.angle_count = try!(stream.read_u16());
        graphic.new_speed = try!(stream.read_f32());
        graphic.frame_rate = try!(stream.read_f32());
        graphic.replay_delay = try!(stream.read_f32());
        graphic.sequence_type = try!(stream.read_u8());
        graphic.id = GraphicId(try!(stream.read_u16()) as isize);
        graphic.mirror_mode = try!(stream.read_u8());
        graphic.deltas = try!(stream.read_array(delta_count, |c| read_delta(c)));

        if attack_sound_used != 0 {
            // three sounds per angle
            let attack_sound_count = 3 * graphic.angle_count as usize;
            graphic.attack_sounds = try!(stream.read_array(attack_sound_count, |c| read_attack_sound(c)));
        }
        graphics.push(graphic);
    }
    Ok(graphics)
}

fn read_delta<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<GraphicDelta> {
    let mut delta: GraphicDelta = Default::default();
    delta.graphic_id = GraphicId(try!(stream.read_i16()) as isize);
    try!(stream.seek(SeekFrom::Current(6))); // skip unknown bytes
    delta.direction_x = try!(stream.read_u16());
    delta.direction_y = try!(stream.read_u16());
    delta.display_angle = try!(stream.read_i16());
    try!(stream.seek(SeekFrom::Current(2))); // skip unknown bytes
    Ok(delta)
}

fn read_attack_sound<R: Read>(stream: &mut R) -> EmpiresDbResult<GraphicAttackSound> {
    let mut attack_sound: GraphicAttackSound = Default::default();
    attack_sound.sound_delay = try!(stream.read_i16());
    attack_sound.sound_group_id = SoundGroupId(try!(stream.read_i16()) as isize);
    Ok(attack_sound)
}
