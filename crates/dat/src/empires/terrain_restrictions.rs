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
pub struct TerrainRestriction {
    // passable/buildable/dmg multiplier?
    some_floats: Vec<f32>,
}

pub fn read_terrain_restrictions<R: Read + Seek>(stream: &mut R,
        terrain_restriction_count: usize, terrain_count: usize)
        -> EmpiresDbResult<Vec<TerrainRestriction>> {
    // Skip terrain restriction pointers
    try!(stream.read_array(terrain_restriction_count, |c| c.read_u32()));

    stream.read_array(terrain_restriction_count, |c| read_terrain_restriction(c, terrain_count))
}

fn read_terrain_restriction<R: Read>(stream: &mut R, terrain_count: usize)
        -> EmpiresDbResult<TerrainRestriction> {
    let mut restriction: TerrainRestriction = Default::default();
    restriction.some_floats = try!(stream.read_array(terrain_count, |c| c.read_f32()));
    Ok(restriction)
}
