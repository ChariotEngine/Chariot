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

use super::border::BorderMatrix;
use super::elevation::ElevationMatrix;
use super::dir;

use dat;
use identifier::{TerrainId, TerrainBorderId};
use scn;

use std::cmp;

pub struct BlendInfo {
    pub terrain_id: TerrainId,
    pub elevation: u8,
    pub border_id: Option<TerrainBorderId>,
    pub border_style: i16,
    pub border_matrix: BorderMatrix,
    pub elevation_matrix: ElevationMatrix,
}

pub struct TerrainBlender<'a> {
    terrain_block: &'a dat::TerrainBlock,
    tiles: &'a [scn::MapTile],
    width: isize,
    height: isize,
}

impl<'a> TerrainBlender<'a> {
    pub fn new(terrain_block: &'a dat::TerrainBlock,
               tiles: &'a [scn::MapTile],
               width: isize,
               height: isize)
               -> TerrainBlender<'a> {
        TerrainBlender {
            terrain_block: terrain_block,
            tiles: tiles,
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    /// Returns tile blend info at the requested row/col while clamping at the edges of the terrain
    /// so that the request cannot go out of bounds. This means that the tiles at the edges
    /// will repeat indefinitely if you query outside of the terrain.
    pub fn blend_at(&self, row: isize, col: isize) -> BlendInfo {
        let tile = &self.tile_at(row, col);
        let terrain = &self.terrain_block.terrains[&TerrainId(tile.terrain_id as isize)];

        // Calculate the border matrix and border id
        let mut border_id = None;
        let mut border_style = 0;
        let mut border_matrix = BorderMatrix::new();
        if let Some((border, border_terrain_id)) = self.determine_border(terrain, row, col) {
            border_id = Some(border.id.as_usize() as u8);
            border_style = border.border_style;
            for direction in &dir::ALL {
                let dir_terrain_id = self.tile_at_relative(row, col, *direction)
                    .terrain_id as usize;
                border_matrix.set_at(*direction,
                                     dir_terrain_id == border_terrain_id.as_usize(),
                                     dir_terrain_id == terrain.id.as_usize());
            }
        }

        // Calculate the elevation matrix
        let mut elevation_matrix = ElevationMatrix::new();
        for direction in &dir::ALL {
            let elevation = self.tile_at_relative(row, col, *direction).elevation;

            let mut elev_dir = 1;
            if elevation == tile.elevation {
                elev_dir = 0;
            } else if elevation < tile.elevation {
                elev_dir = -1;
            }
            elevation_matrix.set_elevation_at(*direction, elev_dir);
        }

        BlendInfo {
            terrain_id: TerrainId(tile.terrain_id as isize),
            elevation: tile.elevation,
            border_id: border_id.map(|b| TerrainBorderId(b as isize)),
            border_style: border_style,
            border_matrix: border_matrix,
            elevation_matrix: elevation_matrix,
        }
    }

    fn determine_border(&self,
                        center_terrain: &dat::Terrain,
                        row: isize,
                        col: isize)
                        -> Option<(&'a dat::TerrainBorder, TerrainId)> {
        for direction in &dir::ALL {
            let border_terrain_id = TerrainId(self.tile_at_relative(row, col, *direction)
                .terrain_id as isize);
            if center_terrain.id != border_terrain_id {
                let border_id = center_terrain.terrain_borders[&border_terrain_id].as_usize();
                let border = &self.terrain_block.terrain_borders[border_id];
                if border.enabled {
                    return Some((border, border_terrain_id));
                }
            }
        }
        None
    }

    fn tile_at(&self, row: isize, col: isize) -> &'a scn::MapTile {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));

        let tile_index = (row * self.width + col) as usize;
        &self.tiles[tile_index]
    }

    fn tile_at_relative(&self, row: isize, col: isize, direction: usize) -> &'a scn::MapTile {
        let row = row + (direction as isize / 3) - 1;
        let col = col + (direction as isize % 3) - 1;
        self.tile_at(row, col)
    }
}
