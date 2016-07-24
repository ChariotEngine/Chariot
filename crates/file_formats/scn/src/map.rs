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

use identifier::TerrainId;
use io_tools::*;

use std::io::Read;

#[derive(Default, Debug)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<MapTile>,
}

#[derive(Default, Debug)]
pub struct MapTile {
    pub terrain_id: TerrainId,
    pub elevation: u8,
    unused: u8,
}

impl Map {
    pub fn read_from_stream<S: Read>(stream: &mut S) -> Result<Map> {
        let mut map = Map {
            width: try!(stream.read_u32()),
            height: try!(stream.read_u32()),
            tiles: Default::default(),
        };
        map.tiles = try!(stream.read_array((map.width * map.height) as usize, |s| read_map_tile(s)));
        Ok(map)
    }
}

fn read_map_tile<S: Read>(stream: &mut S) -> Result<MapTile> {
    Ok(MapTile {
        terrain_id: required_id!(try!(stream.read_i8())),
        elevation: try!(stream.read_u8()),
        unused: try!(stream.read_u8()),
    })
}
