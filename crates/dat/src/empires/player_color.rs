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

use io_tools::*;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct PlayerColor {
    id: u16,
    name: String,
    palette_index: u8,
}

pub fn read_player_colors<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<PlayerColor>> {
    let mut player_colors = Vec::new();

    let color_count = try!(stream.read_u16());
    for _ in 0..color_count {
        let mut color: PlayerColor = Default::default();
        color.name = try!(stream.read_sized_str(30));
        color.id = try!(stream.read_u16());
        try!(stream.read_u16()); // unknown; skip

        color.palette_index = try!(stream.read_u8());
        try!(stream.read_u8()); // unknown byte

        player_colors.push(color);
    }

    Ok(player_colors)
}
