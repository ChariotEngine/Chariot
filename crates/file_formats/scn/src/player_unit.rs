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

use error::*;

use io_tools::*;
use identifier::{SpawnId, UnitId, PlayerId};

use std::io::Read;

#[derive(Default, Debug)]
pub struct PlayerUnit {
    pub position_x: i32,
    pub position_y: i32,
    pub position_z: i32,
    pub spawn_id: SpawnId,
    pub unit_id: UnitId,
    pub state: u8,
    pub rotation: f32,
    pub player_id: PlayerId,
}

impl PlayerUnit {
    // TODO: Implement writing

    pub fn read_from_stream<S: Read>(stream: &mut S) -> Result<PlayerUnit> {
        let mut data: PlayerUnit = Default::default();
        data.position_x = try!(stream.read_f32()) as i32;
        data.position_y = try!(stream.read_f32()) as i32;
        data.position_z = try!(stream.read_f32()) as i32;
        data.spawn_id = SpawnId(try!(stream.read_u32()) as isize);
        data.unit_id = UnitId(try!(stream.read_u16()) as isize);
        data.state = try!(stream.read_u8());
        data.rotation = try!(stream.read_f32());
        Ok(data)
    }
}
