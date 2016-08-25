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

use dat;
use identifier::{TerrainBorderId, TerrainId};
use scn;
use std::cmp;
use super::border::BorderMatrix;
use super::dir;
use super::elevation::ElevationMatrix;
use types::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub terrain_id: TerrainId,
    pub elevation: u8,
    blend_cache_index: Option<u32>,
}

impl Tile {
    #[allow(unused)] // Used from tests
    pub fn new(terrain_id: TerrainId, elevation: u8) -> Tile {
        Tile {
            terrain_id: terrain_id,
            elevation: elevation,
            blend_cache_index: None,
        }
    }

    fn from(scn_tile: &scn::MapTile) -> Tile {
        Tile {
            terrain_id: scn_tile.terrain_id,
            elevation: scn_tile.elevation,
            blend_cache_index: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlendInfo {
    pub terrain_id: TerrainId,
    pub elevation: u8,
    pub border_id: Option<TerrainBorderId>,
    pub border_style: i16,
    pub border_matrix: BorderMatrix,
    pub elevation_matrix: ElevationMatrix,
}

pub struct Terrain {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
    empires: dat::EmpiresDbRef,
    blend_cache: Vec<BlendInfo>,
}

impl Terrain {
    #[cfg(test)]
    pub fn new(width: i32, height: i32, tiles: Vec<Tile>, empires: dat::EmpiresDbRef) -> Terrain {
        Terrain {
            width: width,
            height: height,
            tiles: tiles,
            empires: empires,
            blend_cache: Vec::new(),
        }
    }

    pub fn from(scn_map: &scn::Map, empires: dat::EmpiresDbRef) -> Terrain {
        Terrain {
            width: scn_map.width as i32,
            height: scn_map.height as i32,
            tiles: scn_map.tiles.iter().map(|t| Tile::from(t)).collect(),
            empires: empires,
            blend_cache: Vec::new(),
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Return the (min, max) elevation (inclusive on both ends)
    #[inline(always)]
    pub fn elevation_range(&self) -> (i32, i32) {
        (0, 6)
    }

    pub fn tile_at<'a>(&'a self, world_coord: Vector3) -> &'a Tile {
        self.tile_at_row_col(world_coord.y.into(), world_coord.x.into())
    }

    fn tile_index(&self, row: i32, col: i32) -> usize {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));
        (row * self.width + col) as usize
    }

    pub fn tile_at_row_col<'a>(&'a self, row: i32, col: i32) -> &'a Tile {
        // The index is clamped, so a bounds check is unnecessary
        unsafe { &self.tiles.get_unchecked(self.tile_index(row, col)) }
    }

    fn tile_at_relative<'a>(&'a self, row: i32, col: i32, direction: usize) -> &'a Tile {
        let row = row + (direction as i32 / 3) - 1;
        let col = col + (direction as i32 % 3) - 1;
        self.tile_at_row_col(row, col)
    }

    /// Returns tile blend info at the requested row/col while clamping at the edges of the terrain
    /// so that the request cannot go out of bounds. This means that the tiles at the edges
    /// will repeat indefinitely if you query outside of the terrain.
    pub fn blend_at<'a>(&'a mut self, row: i32, col: i32) -> &'a BlendInfo {
        if let Some(blend_cache_index) = self.tile_at_row_col(row, col).blend_cache_index {
            &self.blend_cache[blend_cache_index as usize]
        } else {
            let blend_info = self.blend_at_no_cache(row, col);
            self.blend_cache.push(blend_info);

            let blend_cache_index = self.blend_cache.len() - 1;
            let tile_index = self.tile_index(row, col);

            self.tiles[tile_index].blend_cache_index = Some(blend_cache_index as u32);
            &self.blend_cache[blend_cache_index]
        }
    }

    fn blend_at_no_cache(&self, row: i32, col: i32) -> BlendInfo {
        let tile = self.tile_at_row_col(row, col);
        let terrain = self.empires.terrain(tile.terrain_id as TerrainId);

        // Calculate the border matrix and border id
        let mut border_id = None;
        let mut border_style = 0;
        let mut border_matrix = BorderMatrix::new();
        if let Some((border, border_terrain_id)) = self.determine_border(terrain, row, col) {
            border_id = Some(border.id);
            border_style = border.border_style;
            for direction in &dir::ALL {
                let dir_terrain_id = self.tile_at_relative(row, col, *direction).terrain_id;
                border_matrix.set_at(*direction,
                                     dir_terrain_id == border_terrain_id,
                                     dir_terrain_id == terrain.id);
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
            terrain_id: tile.terrain_id as TerrainId,
            elevation: tile.elevation,
            border_id: border_id.map(|b| b as TerrainBorderId),
            border_style: border_style,
            border_matrix: border_matrix,
            elevation_matrix: elevation_matrix,
        }
    }

    fn determine_border(&self,
                        center_terrain: &dat::Terrain,
                        row: i32,
                        col: i32)
                        -> Option<(&dat::TerrainBorder, TerrainId)> {
        for direction in &dir::ALL {
            let border_terrain_id = self.tile_at_relative(row, col, *direction).terrain_id;
            if center_terrain.id != border_terrain_id {
                let border_id = center_terrain.terrain_border(border_terrain_id);
                let border = self.empires.terrain_border(border_id);
                if border.enabled {
                    return Some((border, border_terrain_id));
                }
            }
        }
        None
    }
}
