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

use error::*;

use identifier::*;
use io_tools::ReadExt;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct SoundEffect {
    pub file_name: String,

    /// ID of the wav file in sounds.drs
    pub resource_id: WavFileId,

    pub probability: u16,
}

#[derive(Default, Debug)]
pub struct SoundEffectGroup {
    pub id: SoundGroupId,
    pub play_at_update_count: u16,
    pub cache_time: u32,
    pub sound_effects: Vec<SoundEffect>,
}

pub fn read_sound_effect_groups<R: Read + Seek>(stream: &mut R) -> Result<Vec<SoundEffectGroup>> {
    let mut sound_effect_groups = Vec::new();

    let sound_count = try!(stream.read_u16());
    for _ in 0..sound_count {
        let mut sound_group: SoundEffectGroup = Default::default();
        sound_group.id = SoundGroupId(try!(stream.read_u16()) as isize);
        sound_group.play_at_update_count = try!(stream.read_u16());

        let effect_count = try!(stream.read_u16());
        sound_group.cache_time = try!(stream.read_u32());

        for _ in 0..effect_count {
            let mut effect: SoundEffect = Default::default();
            effect.file_name = try!(stream.read_sized_str(13));
            effect.resource_id = WavFileId(try!(stream.read_i32()) as isize);
            effect.probability = try!(stream.read_u16());
            sound_group.sound_effects.push(effect);
        }
        sound_effect_groups.push(sound_group);
    }

    Ok(sound_effect_groups)
}
