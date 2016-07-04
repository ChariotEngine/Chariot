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

use super::dir;
use super::border;
use super::elevation;

use dat::{self, TerrainId};
use scn;

use std::cmp;

static ZERO_INDEX: [u8; 1] = [0];

pub struct TerrainBorder {
    pub border_id: Option<u8>,
    pub border_indices: Option<&'static [u16]>,
    pub elevation_indices: &'static [u8],
}

impl TerrainBorder {
    pub fn new(border_id: Option<u8>,
               border_indices: Option<&'static [u16]>,
               elevation_indices: &'static [u8])
               -> TerrainBorder {
        TerrainBorder {
            border_id: border_id,
            border_indices: border_indices,
            elevation_indices: elevation_indices,
        }
    }
}

/// Cursor for simplifying the loading of terrain from SCN files.
pub struct TerrainCursor<'a> {
    terrain_block: &'a dat::TerrainBlock,
    tiles: &'a [scn::MapTile],
    width: isize,
    height: isize,
    index: isize,
}

impl<'a> TerrainCursor<'a> {
    pub fn new(terrain_block: &'a dat::TerrainBlock,
               tiles: &'a [scn::MapTile],
               width: isize,
               height: isize)
               -> TerrainCursor<'a> {
        TerrainCursor {
            terrain_block: terrain_block,
            tiles: tiles,
            width: width,
            height: height,
            index: 0,
        }
    }

    /// Returns the current tile under examination.
    pub fn current(&self) -> &'a scn::MapTile {
        &self.tiles[self.index as usize]
    }

    /// Returns the current index under examination.
    pub fn index(&self) -> isize {
        self.index
    }

    /// Returns the current row under examination.
    pub fn row(&self) -> isize {
        self.index / self.width
    }

    /// Returns the current column under examination.
    pub fn col(&self) -> isize {
        self.index % self.width
    }

    /// Returns a tile at the requested row/col while clamping at the edges of the terrain
    /// so that the request cannot go out of bounds. This means that the tiles at the edges
    /// will repeat indefinitely if you query outside of the terrain.
    fn at(&self, row: isize, col: isize) -> &'a scn::MapTile {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));
        let index = row * self.width + col;
        &self.tiles[index as usize]
    }

    /// Returns a tile in the requested direction relative to the current position
    /// Has the same clamping as at().
    fn at_relative(&self, direction: usize) -> &'a scn::MapTile {
        let row = self.row() + (direction as isize / 3) - 1;
        let col = self.col() + (direction as isize % 3) - 1;
        self.at(row, col)
    }

    /// Returns the elevation graphic index to use at the current tile
    fn elevation_border(&self) -> &'static [u8] {
        let cur_elevation = self.current().elevation;

        let mut matrix = elevation::ElevationMatrix::new();
        for direction in &dir::ALL {
            let elevation = self.at_relative(*direction).elevation;
            matrix.set_elevation_at(*direction,
                                    if elevation == cur_elevation {
                                        0
                                    } else if elevation < cur_elevation {
                                        -1
                                    } else {
                                        1
                                    });
        }

        if let Some(elevation_match) = elevation::ElevationMatch::find_match(matrix) {
            &elevation_match.elevation_indices
        } else {
            &ZERO_INDEX
        }
    }

    pub fn border(&self) -> TerrainBorder {
        let cur_terrain_id = self.current().terrain_id;
        let cur_terrain = &self.terrain_block.terrains[&TerrainId(cur_terrain_id as isize)];

        // Determine the bordering terrain
        let mut border = None;
        let mut border_id = 0;
        let mut border_terrain_id = 0;
        for direction in &dir::ALL {
            border_terrain_id = self.at_relative(*direction).terrain_id;
            if cur_terrain_id != border_terrain_id {
                border_id = cur_terrain.terrain_borders[&TerrainId(border_terrain_id as isize)]
                    .as_isize() as u8;
                border = Some(&self.terrain_block.terrain_borders[border_id as usize]);
                if border.unwrap().enabled {
                    break;
                }
            }
        }

        let mut matrix = border::BorderMatrix::new();
        for direction in &dir::ALL {
            let terrain_id = self.at_relative(*direction).terrain_id;
            matrix.set_at(*direction,
                          terrain_id == border_terrain_id,
                          terrain_id == cur_terrain_id);
        }

        let mut border_indices: Option<&'static [u16]> = None;
        if border.is_some() && border.unwrap().enabled {
            let border_style = border.unwrap().border_style;

            match border::BorderMatch::find_match(border_style, matrix) {
                Some(border_match) => {
                    border_indices = Some(&border_match.border_indices);
                }
                None => {
                    println!(concat!("Terrain border failed: ",
                                     "bs: {}, r: {}, c: {}, ct: {}, bt: {},\n{:?}\n"),
                             border_style,
                             self.row(),
                             self.col(),
                             cur_terrain_id,
                             border_terrain_id,
                             matrix);
                }
            }
        }

        TerrainBorder::new(if border_indices.is_some() {
                               Some(border_id)
                           } else {
                               None
                           },
                           border_indices,
                           self.elevation_border())
    }

    /// Returns true if there's another tile available to examine.
    pub fn has_next(&self) -> bool {
        return self.index < self.width * self.height;
    }

    /// Advances to the next tile.
    pub fn next(&mut self) {
        self.index += 1;
    }
}
