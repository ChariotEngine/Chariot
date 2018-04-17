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

use error::Result;
use identifier::PlayerId;

use chariot_io_tools::{ReadArrayExt, ReadExt};

use std::io::Read;

#[derive(Default, Debug)]
pub struct PlayerResources {
    pub player_id: PlayerId,
    pub food: f32,
    pub wood: f32,
    pub gold: f32,
    pub stone: f32,
}

impl PlayerResources {
    // TODO: Implement writing

    pub fn read_from_stream<S: Read>(stream: &mut S) -> Result<Vec<PlayerResources>> {
        let mut resources = try!(stream.read_array(8, |s| read_single_from_stream(s)));
        for (index, mut resource) in resources.iter_mut().enumerate() {
            resource.player_id = index.into();
        }
        Ok(resources)
    }
}

fn read_single_from_stream<S: Read>(stream: &mut S) -> Result<PlayerResources> {
    let mut data: PlayerResources = Default::default();
    data.food = try!(stream.read_f32());
    data.wood = try!(stream.read_f32());
    data.gold = try!(stream.read_f32());
    data.stone = try!(stream.read_f32());
    Ok(data)
}
