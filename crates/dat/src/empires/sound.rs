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

use io_tools::ReadExt;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct SoundEffect {
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
pub struct SoundEffectGroup {
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

impl EmpiresDb {
    pub fn read_sounds<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
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
}
