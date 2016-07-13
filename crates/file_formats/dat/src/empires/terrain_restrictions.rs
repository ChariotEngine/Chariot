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
use io_tools::*;

use std::io::prelude::*;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct TerrainRestriction {
    pub id: UnitTerrainRestrictionId,

    /// Map of terrain ID to whether or not that terrain is passable/buildable/matching.
    /// Typically, it's just a 0 for not-passable, and 1 for passable.
    /// However, there are complicated cases such as the dock where multiple different types
    /// of terrain have to be strattled (i.e., with a dock: land, beach, shallows, and beach).
    /// In these cases, each terrain will have a value like 0.2, and presumably, when all of the
    /// terrain types under the unit are summed together and reach a threshold, then it's passable.
    pub passability_map: BTreeMap<TerrainId, f32>,
}

pub fn read_terrain_restrictions<R: Read + Seek>(stream: &mut R,
        terrain_restriction_count: usize, terrain_count: usize)
        -> Result<Vec<TerrainRestriction>> {
    // Skip terrain restriction pointers
    try!(stream.read_array(terrain_restriction_count, |c| c.read_u32()));

    let mut restrictions = try!(stream.read_array(terrain_restriction_count,
        |c| read_terrain_restriction(c, terrain_count)));
    for (index, terrain_restriction) in restrictions.iter_mut().enumerate() {
        terrain_restriction.id = UnitTerrainRestrictionId::from_index(index);
    }
    Ok(restrictions)
}

fn read_terrain_restriction<R: Read>(stream: &mut R, terrain_count: usize)
        -> Result<TerrainRestriction> {
    let mut restriction: TerrainRestriction = Default::default();

    let values = try!(stream.read_array(terrain_count, |c| c.read_f32()));
    for (index, value) in values.iter().enumerate() {
        restriction.passability_map.insert(TerrainId(index as isize), *value);
    }

    Ok(restriction)
}
