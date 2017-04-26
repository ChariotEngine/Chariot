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

use identifier::{PlayerColorId, SlpFileId};
use resource::DrsKey;
use specs;
use types::Fixed;

#[derive(Clone, Debug)]
pub struct DecalComponent {
    pub player_color_id: PlayerColorId,
    pub drs_key: DrsKey,
    pub slp_file_id: SlpFileId,
    pub frame: u16,
    pub frame_time: Fixed,
}

impl specs::Component for DecalComponent {
    type Storage = specs::HashMapStorage<DecalComponent>;
}

impl DecalComponent {
    pub fn new(player_color_id: PlayerColorId, drs_key: DrsKey, slp_file_id: SlpFileId) -> DecalComponent {
        DecalComponent {
            player_color_id: player_color_id,
            drs_key: drs_key,
            slp_file_id: slp_file_id,
            frame: 0u16,
            frame_time: 0.into(),
        }
    }
}
